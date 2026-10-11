package com.gemwallet.android.model

import androidx.compose.runtime.Immutable
import com.wallet.core.primitives.Chain
import uniffi.gemstone.GemWalletImportKind

@Immutable
data class ImportType(val kind: GemWalletImportKind, val chain: Chain? = null) {
    companion object {
        fun phrase(chain: Chain? = null): ImportType = ImportType(GemWalletImportKind.PHRASE, chain)
    }
}
