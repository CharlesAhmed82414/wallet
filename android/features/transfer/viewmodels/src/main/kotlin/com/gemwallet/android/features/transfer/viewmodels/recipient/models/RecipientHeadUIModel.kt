package com.gemwallet.android.features.transfer.viewmodels.recipient.models

import androidx.compose.runtime.Immutable
import com.wallet.core.primitives.NFTAsset
import uniffi.gemstone.GemAssetText

@Immutable
sealed interface RecipientHeadUIModel {
    data class Nft(val nftAsset: NFTAsset) : RecipientHeadUIModel
    data class Asset(val text: GemAssetText) : RecipientHeadUIModel
}
