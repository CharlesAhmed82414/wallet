// Copyright (c). Gem Wallet. All rights reserved.

import Foundation
import Style
import SwiftUI

public struct NftImageView: View {
    @Environment(\.displayScale) private var displayScale

    private let assetImage: AssetImage
    @Binding private var isImageLoaded: Bool

    public init(
        assetImage: AssetImage,
        isImageLoaded: Binding<Bool> = .constant(false),
    ) {
        self.assetImage = assetImage
        _isImageLoaded = isImageLoaded
    }

    public var body: some View {
        GeometryReader { geometry in
            CachedAsyncImage(url: assetImage.imageURL, size: max(geometry.size.width, geometry.size.height), scale: displayScale) { phase in
                switch phase {
                case .empty:
                    if assetImage.imageURL != nil {
                        ZStack {
                            Color(.systemGray5)
                            if assetImage.placeholder != nil {
                                AssetImageView(
                                    assetImage: assetImage,
                                    size: .image.large,
                                )
                            } else {
                                LoadingView()
                            }
                        }
                    } else {
                        NftImagePlaceholderView(name: assetImage.type?.value)
                    }
                case let .success(image):
                    image.resizable()
                        .onAppear { isImageLoaded = true }
                case .failure:
                    NftImagePlaceholderView(name: assetImage.type?.value)
                @unknown default:
                    NftImagePlaceholderView(name: assetImage.type?.value)
                }
            }
            .frame(width: geometry.size.width, height: geometry.size.height)
        }
    }
}
