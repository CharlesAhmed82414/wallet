package com.gemwallet.android.ui.models.navigation

import androidx.compose.runtime.Immutable
import uniffi.gemstone.GemErrorText

@Immutable
sealed interface RouteMessage {
    data class Toast(val text: String) : RouteMessage

    data class Error(val error: GemErrorText) : RouteMessage
}
