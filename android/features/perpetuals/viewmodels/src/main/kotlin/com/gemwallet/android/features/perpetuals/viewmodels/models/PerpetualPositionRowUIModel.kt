package com.gemwallet.android.features.perpetuals.viewmodels.models

import androidx.compose.runtime.Immutable
import com.wallet.core.primitives.Asset
import uniffi.gemstone.GemAssetItemRow

@Immutable
data class PerpetualPositionRowUIModel(val asset: Asset, val row: GemAssetItemRow, val hideBalance: Boolean = false)
