// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import struct Gemstone.ChartCandleStick
import struct Gemstone.GemCandleChart
import Style

struct CandlestickPlot {
    private enum Metrics {
        static let topInset: CGFloat = Spacing.small
        static let leadingInset: CGFloat = Spacing.small
        static let volumeBandFraction: CGFloat = 0.18
        static let volumeBandGap: CGFloat = Spacing.tiny
        static let wickWidthRatio: CGFloat = 0.12
    }

    let chart: GemCandleChart
    let frame: CGRect
    let priceRange: ClosedRange<Double>
    let volumeRange: ClosedRange<Double>

    init(chart: GemCandleChart, size: CGSize, priceColumnWidth: CGFloat, timeRowHeight: CGFloat, priceRange: ClosedRange<Double>, volumeRange: ClosedRange<Double>) {
        self.chart = chart
        self.priceRange = priceRange
        self.volumeRange = volumeRange
        frame = CGRect(x: Metrics.leadingInset, y: Metrics.topInset, width: max(1, size.width - Metrics.leadingInset - priceColumnWidth), height: max(1, size.height - Metrics.topInset - timeRowHeight))
    }

    var volumeBand: CGRect {
        let height = frame.height * Metrics.volumeBandFraction
        return CGRect(x: frame.minX, y: frame.maxY - height, width: frame.width, height: height)
    }

    var bodyWidth: CGFloat {
        max(1, chart.bodyWidth * frame.width)
    }

    var hasVolume: Bool {
        volumeRange.upperBound > 0
    }

    var showsCurrentPrice: Bool {
        shows(chart.layout.currentPrice.value)
    }

    func shows(_ price: Double) -> Bool {
        priceRange.contains(price)
    }

    func lineBadges(for prices: [Double], widths: [CGFloat], height: CGFloat, rowSpacing: CGFloat, gap: CGFloat) -> [CGRect] {
        let centers = badgeCenters(for: prices, height: height, rowSpacing: rowSpacing)
        var badges = [CGRect](repeating: .zero, count: prices.count)
        var previous: CGRect?
        for index in prices.indices.sorted(by: { centers[$0] < centers[$1] }) {
            var badge = CGRect(x: frame.minX, y: centers[index] - height / 2, width: widths[index], height: height)
            if let previous, previous.maxY > badge.minY {
                badge.origin = CGPoint(x: previous.maxX + gap, y: max(badge.minY, previous.minY))
                if badge.maxX > frame.maxX {
                    badge.origin = CGPoint(x: frame.minX, y: previous.maxY + rowSpacing)
                }
            }
            badges[index] = badge
            previous = badge
        }
        return badges
    }

    func x(for date: Date) -> CGFloat {
        frame.minX + CGFloat(date.timeIntervalSince(chart.start) / chart.end.timeIntervalSince(chart.start)) * frame.width
    }

    func y(for price: Double) -> CGFloat {
        let bottom = volumeBand.minY - Metrics.volumeBandGap
        return bottom - CGFloat((price - priceRange.lowerBound) / (priceRange.upperBound - priceRange.lowerBound)) * (bottom - frame.minY)
    }

    func body(of candle: ChartCandleStick) -> CGRect {
        let top = y(for: max(candle.open, candle.close))
        return CGRect(x: x(for: candle.date) - bodyWidth / 2, y: top, width: bodyWidth, height: max(1, y(for: min(candle.open, candle.close)) - top))
    }

    func wick(of candle: ChartCandleStick) -> CGRect {
        let width = min(max(bodyWidth * Metrics.wickWidthRatio, .space1), .space2)
        let high = y(for: candle.high)
        return CGRect(x: x(for: candle.date) - width / 2, y: high, width: width, height: max(width, y(for: candle.low) - high))
    }

    func volume(of candle: ChartCandleStick) -> CGRect {
        let band = volumeBand
        let height = max(1, band.height * CGFloat(min(candle.volume / volumeRange.upperBound, 1)))
        return CGRect(x: x(for: candle.date) - bodyWidth / 2, y: band.maxY - height, width: bodyWidth, height: height)
    }

    private func badgeCenters(for prices: [Double], height: CGFloat, rowSpacing: CGFloat) -> [CGFloat] {
        let top = frame.minY + height / 2
        let bottom = y(for: priceRange.lowerBound) - height / 2
        let above = prices.indices.filter { prices[$0] > priceRange.upperBound }.sorted { prices[$0] > prices[$1] }
        let below = prices.indices.filter { prices[$0] < priceRange.lowerBound }.sorted { prices[$0] < prices[$1] }
        var centers = prices.map { min(max(y(for: $0), top), bottom) }
        for (slot, index) in above.enumerated() {
            centers[index] = top + CGFloat(slot) * (height + rowSpacing)
        }
        for (slot, index) in below.enumerated() {
            centers[index] = bottom - CGFloat(slot) * (height + rowSpacing)
        }
        return centers
    }
}
