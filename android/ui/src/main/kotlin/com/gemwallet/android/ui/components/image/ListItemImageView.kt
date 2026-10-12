package com.gemwallet.android.ui.components.image

import androidx.annotation.DrawableRes
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.rememberTextMeasurer
import androidx.compose.ui.unit.Dp
import coil3.compose.AsyncImage
import coil3.request.CachePolicy
import coil3.request.ImageRequest
import coil3.request.transformations
import coil3.transform.CircleCropTransformation
import com.gemwallet.android.ext.toChain
import com.gemwallet.android.ui.components.list_item.ListItemImage
import com.gemwallet.android.ui.components.list_item.ListItemImageStyle
import com.gemwallet.android.ui.components.list_item.ListItemSymbol
import com.gemwallet.android.ui.components.list_item.color
import com.gemwallet.android.ui.icons.AppIcons
import com.gemwallet.android.ui.theme.actionIconGlyphSize
import com.gemwallet.android.ui.theme.secondaryFaded
import com.gemwallet.android.ui.theme.space12
import uniffi.gemstone.GemAssetIcon
import uniffi.gemstone.GemAssetIconImage
import java.io.File

private val CircleCrop = CircleCropTransformation()

@Composable
fun ListItemImageView(image: ListItemImage, modifier: Modifier = Modifier, style: ListItemImageStyle = image.style, size: Dp = style.size) {
    when (image) {
        is ListItemImage.Asset -> AssetIconImage(icon = image.icon, size = size, modifier = modifier)

        is ListItemImage.Url -> CoilImage(data = image.url, size = size, placeholderText = image.placeholder, modifier = modifier)

        is ListItemImage.Stored -> CoilImage(
            data = File(LocalContext.current.filesDir, image.name),
            size = size,
            placeholderText = image.placeholder,
            modifier = modifier,
        )

        is ListItemImage.Emoji -> EmojiView(
            emoji = image.glyph,
            modifier = modifier.size(size),
            background = image.backgroundColor?.let { Color(it) } ?: Color.Transparent,
            scale = AvatarScale.EMOJI,
        )

        is ListItemImage.Initials -> InitialsAvatar(text = image.text, size = size, modifier = modifier, placeholder = AppIcons.Person)

        is ListItemImage.Symbol -> if (style == ListItemImageStyle.Action) {
            Box(
                modifier = modifier
                    .size(size)
                    .background(color = MaterialTheme.colorScheme.primary, shape = RoundedCornerShape(space12)),
                contentAlignment = Alignment.Center,
            ) {
                Icon(
                    imageVector = image.symbol.vector(),
                    contentDescription = null,
                    modifier = Modifier.size(actionIconGlyphSize),
                    tint = MaterialTheme.colorScheme.onPrimary,
                )
            }
        } else {
            Icon(
                imageVector = image.symbol.vector(),
                contentDescription = null,
                modifier = modifier.size(size),
                tint = image.tint.color(),
            )
        }

        is ListItemImage.Drawable -> BundledImage(resource = image.id, size = size, isRounded = style.isRounded, modifier = modifier)
    }
}

@Composable
private fun AssetIconImage(icon: GemAssetIcon, size: Dp, modifier: Modifier) {
    when (val image = icon.image) {
        is GemAssetIconImage.Local -> image.chain.toChain().iconResource()?.let { BundledImage(resource = it, size = size, isRounded = true, modifier = modifier) }
        is GemAssetIconImage.LocalToken -> BundledImage(resource = image.token.iconResource(), size = size, isRounded = true, modifier = modifier)
        is GemAssetIconImage.Remote -> CoilImage(data = image.url, size = size, placeholderText = icon.placeholder, modifier = modifier)
    }
}

@Composable
private fun BundledImage(@DrawableRes resource: Int, size: Dp, isRounded: Boolean, modifier: Modifier) {
    Image(
        painter = painterResource(resource),
        contentDescription = null,
        modifier = if (isRounded) modifier.size(size).clip(CircleShape) else modifier.size(size),
    )
}

@Composable
private fun CoilImage(data: Any, size: Dp, placeholderText: String?, modifier: Modifier) {
    val context = LocalContext.current
    val density = LocalDensity.current
    val placeholderColor = MaterialTheme.colorScheme.secondaryFaded
    val textMeasurer = rememberTextMeasurer()
    val placeholderSize = with(density) { Size(size.toPx(), size.toPx()) }
    val placeholder = remember(placeholderText, placeholderColor, placeholderSize) {
        if (placeholderText.isNullOrEmpty()) {
            null
        } else {
            TextPainter(
                circleColor = placeholderColor,
                textMeasurer = textMeasurer,
                text = placeholderText,
                circleSize = placeholderSize,
            )
        }
    }
    val request = remember(context, data) {
        ImageRequest.Builder(context)
            .data(data)
            .diskCachePolicy(policy = CachePolicy.ENABLED)
            .networkCachePolicy(policy = CachePolicy.ENABLED)
            .transformations(CircleCrop)
            .build()
    }
    AsyncImage(
        model = request,
        placeholder = placeholder,
        error = placeholder,
        contentDescription = null,
        modifier = modifier.size(size),
    )
}

@Composable
fun ListItemSymbol.vector(): ImageVector = when (this) {
    ListItemSymbol.Check -> AppIcons.Check
    ListItemSymbol.Paste -> AppIcons.ContentPaste
    ListItemSymbol.QrScanner -> AppIcons.QrCodeScanner
    ListItemSymbol.Pin -> AppIcons.PushPin
    ListItemSymbol.Unpin -> AppIcons.KeepOff
    ListItemSymbol.AddCircle -> AppIcons.AddCircleOutlined
    ListItemSymbol.Buy -> AppIcons.Buy
    ListItemSymbol.Swap -> AppIcons.SwapVert
    ListItemSymbol.Receive -> AppIcons.Receive
    ListItemSymbol.Add -> AppIcons.Add
    ListItemSymbol.Article -> AppIcons.Article
    ListItemSymbol.Error -> AppIcons.Error
    ListItemSymbol.CheckCircle -> AppIcons.CheckCircle
    ListItemSymbol.Notifications -> AppIcons.Notifications
    ListItemSymbol.NotificationsOutlined -> AppIcons.NotificationsOutlined
    ListItemSymbol.Warning -> AppIcons.Warning
    ListItemSymbol.CurrencyBitcoin -> AppIcons.CurrencyBitcoin
    ListItemSymbol.Close -> AppIcons.Close
}
