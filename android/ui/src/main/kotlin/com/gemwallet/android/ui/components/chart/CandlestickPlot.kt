package com.gemwallet.android.ui.components.chart

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.geometry.Size
import uniffi.gemstone.ChartCandleStick
import uniffi.gemstone.GemCandleChart
import kotlin.math.max
import kotlin.math.min

private const val VOLUME_BAND_FRACTION = 0.18f
private const val WICK_WIDTH_RATIO = 0.12f

internal class CandlestickPlot(
    private val chart: GemCandleChart,
    val frame: Rect,
    volumeBandGap: Float,
    private val minWickWidth: Float,
    private val maxWickWidth: Float,
    private val priceViewport: ChartViewport,
    private val volumeViewport: ChartViewport?,
) {
    private val span = (chart.end - chart.start).coerceAtLeast(1L).toFloat()

    val volumeBand = Rect(frame.left, frame.bottom - frame.height * VOLUME_BAND_FRACTION, frame.right, frame.bottom)
    val priceBottom = volumeBand.top - volumeBandGap
    val bodyWidth = max(1f, chart.bodyWidth.toFloat() * frame.width)

    fun shows(price: Double): Boolean = y(price) in frame.top..priceBottom

    fun lineBadges(prices: List<Double>, widths: List<Float>, height: Float, rowSpacing: Float, gap: Float): List<Rect> {
        val centers = badgeCenters(prices, height, rowSpacing)
        val badges = MutableList(prices.size) { Rect.Zero }
        var previous: Rect? = null
        for (index in prices.indices.sortedBy { centers[it] }) {
            var badge = Rect(Offset(frame.left, centers[index] - height / 2f), Size(widths[index], height))
            if (previous != null && previous.bottom > badge.top) {
                badge = Rect(Offset(previous.right + gap, max(badge.top, previous.top)), badge.size)
                if (badge.right > frame.right) {
                    badge = Rect(Offset(frame.left, previous.bottom + rowSpacing), badge.size)
                }
            }
            badges[index] = badge
            previous = badge
        }
        return badges
    }

    fun x(date: Long): Float = frame.left + (date - chart.start) / span * frame.width

    fun y(price: Double): Float = frame.top + priceViewport.y(price, priceBottom - frame.top)

    fun body(candle: ChartCandleStick): Rect {
        val top = y(max(candle.open, candle.close))
        return Rect(x(candle.date) - bodyWidth / 2f, top, x(candle.date) + bodyWidth / 2f, max(top + 1f, y(min(candle.open, candle.close))))
    }

    fun wick(candle: ChartCandleStick): Rect {
        val width = (bodyWidth * WICK_WIDTH_RATIO).coerceIn(minWickWidth, maxWickWidth)
        return Rect(x(candle.date) - width / 2f, y(candle.high), x(candle.date) + width / 2f, y(candle.low))
    }

    fun volume(candle: ChartCandleStick): Rect? {
        val viewport = volumeViewport ?: return null
        val height = max(1f, volumeBand.height * min(viewport.fraction(candle.volume), 1f))
        return Rect(x(candle.date) - bodyWidth / 2f, volumeBand.bottom - height, x(candle.date) + bodyWidth / 2f, volumeBand.bottom)
    }

    private fun badgeCenters(prices: List<Double>, height: Float, rowSpacing: Float): List<Float> {
        val top = frame.top + height / 2f
        val bottom = priceBottom - height / 2f
        val above = prices.indices.filter { y(prices[it]) < frame.top }.sortedByDescending { prices[it] }
        val below = prices.indices.filter { y(prices[it]) > priceBottom }.sortedBy { prices[it] }
        val centers = prices.mapTo(mutableListOf()) { min(max(y(it), top), bottom) }
        above.forEachIndexed { slot, index -> centers[index] = top + slot * (height + rowSpacing) }
        below.forEachIndexed { slot, index -> centers[index] = bottom - slot * (height + rowSpacing) }
        return centers
    }
}
