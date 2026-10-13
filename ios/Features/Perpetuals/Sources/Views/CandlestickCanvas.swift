// Copyright (c). Gem Wallet. All rights reserved.

import struct Gemstone.ChartCandleStick
import struct Gemstone.GemFormattedNumber
import struct Gemstone.GemPerpetualChartLayout
import PrimitivesComponents
import Style
import SwiftUI

struct CandlestickCanvas: View {
    private enum Metrics {
        static let labelGap: CGFloat = .small
        static let labelRowSpacing: CGFloat = .space2
        static let chevronWidth: CGFloat = .space8
        static let chevronHeight: CGFloat = .space4
        static let chevronGap: CGFloat = .space4
        static let chevronLineWidth: CGFloat = 1.5
        static let lineStyle = StrokeStyle(lineWidth: .space1, dash: [.space4, .space4])
        static let currentPriceStyle = StrokeStyle(lineWidth: .space1, dash: [.space2, .space2])
        static let selectionStyle = StrokeStyle(lineWidth: .space1, dash: [.space4])
        static let cornerRatio: CGFloat = 0.15
        static let axisTickLength: CGFloat = .space4
        static let priceLabelOffset: CGFloat = axisTickLength + .extraSmall
        static let pricePillPadding: CGFloat = .extraSmall
        static let priceColumnTrailing: CGFloat = .small
        static let selectionDotSize: CGFloat = .space12

        static var priceFont: UIFont {
            .monospacedDigitSystemFont(ofSize: Font.app.captionSize, weight: .regular)
        }
    }

    let plot: CandlestickPlot
    let selectedCandle: ChartCandleStick?

    private let dateFormatter = ChartDateFormatter()

    static func priceColumnWidth(for layout: GemPerpetualChartLayout) -> CGFloat {
        let font = Metrics.priceFont
        let textWidth = (layout.levels + [layout.currentPrice]).reduce(CGFloat.zero) { width, price in
            max(width, (price.text() as NSString).size(withAttributes: [.font: font]).width)
        }
        return Metrics.priceLabelOffset + textWidth.rounded(.up) + Metrics.priceColumnTrailing
    }

    var body: some View {
        Canvas { context, _ in
            let levels = plot.chart.layout.levels
            drawGrid(levels, in: &context)
            drawVolumes(in: &context)
            drawCandles(in: &context)
            drawLines(in: &context)
            drawSelection(in: &context)
            drawPriceLabels(levels, in: &context)
            drawTimeLabels(in: &context)
        }
    }
}

// MARK: - Drawing

extension CandlestickCanvas {
    private func drawGrid(_ levels: [GemFormattedNumber], in context: inout GraphicsContext) {
        let frame = plot.frame
        for level in levels {
            let y = plot.y(for: level.value)
            context.stroke(line(from: CGPoint(x: frame.minX, y: y), to: CGPoint(x: frame.maxX, y: y)), with: .color(ChartGridStyle.color), style: ChartGridStyle.strokeStyle)
            context.stroke(line(from: CGPoint(x: frame.maxX, y: y), to: CGPoint(x: frame.maxX + Metrics.axisTickLength, y: y)), with: .color(ChartGridStyle.color), lineWidth: ChartGridStyle.lineWidth)
        }
        for tick in plot.chart.xTicks {
            let x = plot.x(for: tick.date)
            context.stroke(line(from: CGPoint(x: x, y: frame.minY), to: CGPoint(x: x, y: frame.maxY)), with: .color(ChartGridStyle.color), style: ChartGridStyle.strokeStyle)
        }
        if plot.showsCurrentPrice {
            let y = plot.y(for: plot.chart.layout.currentPrice.value)
            context.stroke(line(from: CGPoint(x: frame.minX, y: y), to: CGPoint(x: frame.maxX, y: y)), with: .color(Colors.gray.opacity(.semiStrong)), style: Metrics.currentPriceStyle)
        }
    }

    private func drawVolumes(in context: inout GraphicsContext) {
        guard plot.hasVolume else { return }
        let bars = plot.chart.candles.filter { $0.volume > 0 }.map(plot.volume(of:)).reduce(into: Path()) { path, bar in
            path.addRoundedRect(in: bar, cornerSize: cornerSize(of: bar))
        }
        context.drawLayer { layer in
            layer.clip(to: Path(plot.frame))
            layer.fill(bars, with: .color(Colors.gray.opacity(.opacity25)))
        }
    }

    private func drawCandles(in context: inout GraphicsContext) {
        let paths = Dictionary(grouping: zip(plot.chart.candles, plot.chart.layout.tones), by: \.1).mapValues { pairs in
            pairs.map(\.0).reduce(into: Path()) { path, candle in
                let wick = plot.wick(of: candle)
                let body = plot.body(of: candle)
                path.addRoundedRect(in: wick, cornerSize: CGSize(width: wick.width / 2, height: wick.width / 2))
                path.addRoundedRect(in: body, cornerSize: cornerSize(of: body))
            }
        }
        context.drawLayer { layer in
            layer.clip(to: Path(plot.frame))
            for (tone, path) in paths {
                layer.fill(path, with: .color(tone.color))
            }
        }
    }

    private func drawLines(in context: inout GraphicsContext) {
        let lines = plot.chart.layout.lines
        let prices = lines.map(\.price.value)
        let labels = lines.map { context.resolve(Text($0.label.text).font(.app.caption).foregroundStyle(Colors.whiteSolid)) }
        let sizes = labels.map { $0.measure(in: plot.frame.size) }
        let widths = prices.indices.map { sizes[$0].width + .tiny * 2 + (plot.shows(prices[$0]) ? .zero : Metrics.chevronWidth + .tiny) }
        let height = (sizes.map(\.height).max() ?? .zero) + .tiny * 2
        let badges = plot.lineBadges(for: prices, widths: widths, height: height, rowSpacing: Metrics.labelRowSpacing, gap: Metrics.labelGap)
        for line in lines where plot.shows(line.price.value) {
            let y = plot.y(for: line.price.value)
            context.stroke(self.line(from: CGPoint(x: plot.frame.minX, y: y), to: CGPoint(x: plot.frame.maxX, y: y)), with: .color(line.kind.color.opacity(.semiStrong)), style: Metrics.lineStyle)
        }
        for (index, line) in lines.enumerated() {
            context.fill(Path(roundedRect: badges[index], cornerRadius: .tiny), with: .color(line.kind.color))
            context.draw(labels[index], at: CGPoint(x: badges[index].minX + .tiny, y: badges[index].midY), anchor: .leading)
            if !plot.shows(line.price.value) {
                drawChevrons(at: CGPoint(x: badges[index].maxX - .tiny - Metrics.chevronWidth / 2, y: badges[index].midY), pointingUp: line.price.value > plot.priceRange.upperBound, in: &context)
            }
        }
    }

    private func drawChevrons(at center: CGPoint, pointingUp: Bool, in context: inout GraphicsContext) {
        let direction: CGFloat = pointingUp ? -1 : 1
        let path = [-Metrics.chevronGap / 2, Metrics.chevronGap / 2].reduce(into: Path()) { path, offset in
            let tipY = center.y + offset + direction * Metrics.chevronHeight / 2
            path.move(to: CGPoint(x: center.x - Metrics.chevronWidth / 2, y: tipY - direction * Metrics.chevronHeight))
            path.addLine(to: CGPoint(x: center.x, y: tipY))
            path.addLine(to: CGPoint(x: center.x + Metrics.chevronWidth / 2, y: tipY - direction * Metrics.chevronHeight))
        }
        context.stroke(path, with: .color(Colors.whiteSolid), style: StrokeStyle(lineWidth: Metrics.chevronLineWidth, lineCap: .round, lineJoin: .round))
    }

    private func drawSelection(in context: inout GraphicsContext) {
        guard let selectedCandle else { return }
        let x = plot.x(for: selectedCandle.date)
        context.stroke(line(from: CGPoint(x: x, y: plot.frame.minY), to: CGPoint(x: x, y: plot.frame.maxY)), with: .color(Colors.blue), style: Metrics.selectionStyle)
        let dot = CGRect(x: x - Metrics.selectionDotSize / 2, y: plot.y(for: selectedCandle.close) - Metrics.selectionDotSize / 2, width: Metrics.selectionDotSize, height: Metrics.selectionDotSize)
        context.fill(Path(ellipseIn: dot), with: .color(Colors.white))
        context.stroke(Path(ellipseIn: dot.insetBy(dx: .space1, dy: .space1)), with: .color(Colors.blue), lineWidth: .space2)
    }

    private func drawPriceLabels(_ levels: [GemFormattedNumber], in context: inout GraphicsContext) {
        let labelX = plot.frame.maxX + Metrics.priceLabelOffset
        let font = Font(Metrics.priceFont)
        for level in levels {
            let label = context.resolve(Text(level.text()).font(font).foregroundStyle(Colors.gray))
            context.draw(label, at: CGPoint(x: labelX, y: plot.y(for: level.value)), anchor: .leading)
        }
        guard plot.showsCurrentPrice else { return }
        let currentPrice = plot.chart.layout.currentPrice
        let label = context.resolve(Text(currentPrice.text()).font(font).foregroundStyle(Colors.whiteSolid))
        let size = label.measure(in: plot.frame.size)
        let pill = CGRect(x: labelX - Metrics.pricePillPadding, y: plot.y(for: currentPrice.value) - size.height / 2 - .space1, width: size.width + Metrics.pricePillPadding * 2, height: size.height + .space1 * 2)
        context.fill(Path(roundedRect: pill, cornerRadius: Spacing.tiny), with: .color(plot.chart.layout.currentTone.color))
        context.draw(label, at: CGPoint(x: pill.midX, y: pill.midY), anchor: .center)
    }

    private func drawTimeLabels(in context: inout GraphicsContext) {
        for tick in plot.chart.xTicks {
            let label = context.resolve(Text(dateFormatter.string(for: tick)).font(.system(size: Font.app.captionSize)).foregroundStyle(Colors.gray))
            context.draw(label, at: CGPoint(x: plot.x(for: tick.date), y: plot.frame.maxY + Spacing.small), anchor: .top)
        }
    }

    private func cornerSize(of rect: CGRect) -> CGSize {
        let radius = min(plot.bodyWidth * Metrics.cornerRatio, rect.height / 2)
        return CGSize(width: radius, height: radius)
    }

    private func line(from start: CGPoint, to end: CGPoint) -> Path {
        Path { path in
            path.move(to: start)
            path.addLine(to: end)
        }
    }
}
