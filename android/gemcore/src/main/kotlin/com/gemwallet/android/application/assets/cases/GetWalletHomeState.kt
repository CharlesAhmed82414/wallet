package com.gemwallet.android.application.assets.cases

import kotlinx.coroutines.flow.StateFlow
import uniffi.gemstone.GemNftEntry
import uniffi.gemstone.GemWalletHomeViewState

interface GetWalletHomeState {
    fun walletHomeState(): StateFlow<GemWalletHomeViewState?>

    fun walletCollections(): StateFlow<List<GemNftEntry>>
}
