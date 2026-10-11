package com.gemwallet.android.ui.models

import androidx.compose.runtime.Immutable
import com.wallet.core.primitives.ChartPeriod
import uniffi.gemstone.GemChartData

@Immutable
data class ChartUIState(val period: ChartPeriod = ChartPeriod.Day, val chart: StateViewType<GemChartData> = StateViewType.Loading)
