package com.gemwallet.android.ui.components.list_item

import com.gemwallet.android.testkit.mockGemWalletRow
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.style.badgeImage
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import uniffi.gemstone.GemWalletPlaceholder
import uniffi.gemstone.GemWalletSubtitle

class WalletItemTest {

    @Test
    fun `a row with the watch badge points at the badge drawable`() {
        val row = mockGemWalletRow(id = "wallet-1", name = "Wallet", subtitle = GemWalletSubtitle.Multicoin, placeholder = GemWalletPlaceholder.Multicoin, showsWatchBadge = true)

        assertEquals(ListItemImage.Drawable(R.drawable.watch_badge, style = ListItemImageStyle.Avatar), row.badgeImage())
    }

    @Test
    fun `a row without the watch badge has no badge`() {
        val row = mockGemWalletRow(id = "wallet-1", name = "Wallet", subtitle = GemWalletSubtitle.Multicoin, placeholder = GemWalletPlaceholder.Multicoin)

        assertNull(row.badgeImage())
    }
}
