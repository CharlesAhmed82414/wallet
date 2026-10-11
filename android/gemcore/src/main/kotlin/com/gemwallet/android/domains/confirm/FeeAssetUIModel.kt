package com.gemwallet.android.domains.confirm

import androidx.compose.runtime.Immutable
import com.gemwallet.android.ext.toPrimitives
import com.wallet.core.primitives.Asset
import uniffi.gemstone.GemAssetItemRow
import uniffi.gemstone.GemFeeAsset

@Immutable
data class FeeAssetUIModel(val asset: Asset, val row: GemAssetItemRow)

fun GemFeeAsset.toFeeAssetUIModel(): FeeAssetUIModel = FeeAssetUIModel(asset.toPrimitives(), row)
