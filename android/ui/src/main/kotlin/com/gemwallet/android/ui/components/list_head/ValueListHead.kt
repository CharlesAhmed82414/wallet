package com.gemwallet.android.ui.components.list_head

import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import com.gemwallet.android.ui.components.HideToggle
import com.gemwallet.android.ui.components.list_item.ListItemTextStyle
import com.gemwallet.android.ui.localization.string
import com.gemwallet.android.ui.style.textStyle
import uniffi.gemstone.GemValueHeader

@Composable
fun ValueListHead(header: GemValueHeader, hideToggle: HideToggle? = null, onClick: (() -> Unit)? = null, onSubtitleClick: (() -> Unit)? = null, actions: (@Composable () -> Unit)? = null) {
    val context = LocalContext.current
    val subtitle = header.subtitle?.text?.string(context)
    AmountListHead(
        amount = header.title.string(context),
        hideToggle = hideToggle,
        equivalent = subtitle.takeIf { header.subtitleIcon == null },
        icon = header.icon,
        changedValue = subtitle.takeIf { header.subtitleIcon != null },
        changeStyle = header.subtitle?.tone?.textStyle() ?: ListItemTextStyle.Secondary,
        onClick = onClick,
        onSubtitleClick = onSubtitleClick,
        actions = actions,
    )
}
