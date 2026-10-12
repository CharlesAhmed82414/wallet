package com.gemwallet.android.features.widgets

import android.content.Context
import android.graphics.Bitmap
import coil3.imageLoader
import coil3.request.ImageRequest
import coil3.request.SuccessResult
import coil3.toBitmap
import com.gemwallet.android.data.services.gemstone.di.WidgetEntryPoint
import com.gemwallet.android.ext.toChain
import com.gemwallet.android.ext.toPrimitives
import com.gemwallet.android.model.text
import com.gemwallet.android.ui.components.image.iconResource
import com.gemwallet.android.ui.components.list_item.ListItemTextStyle
import com.gemwallet.android.ui.style.textStyle
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.withContext
import uniffi.gemstone.GemAssetIcon
import uniffi.gemstone.GemAssetIconImage
import uniffi.gemstone.GemWidgetCoin
import uniffi.gemstone.GemWidgetSize

data class WidgetCoinUIModel(val name: String, val symbol: String, val priceText: String, val changeText: String, val changeStyle: ListItemTextStyle, val icon: Bitmap?)

suspend fun WidgetEntryPoint.mediumWidgetCoins(context: Context): List<WidgetCoinUIModel> {
    val currency = preferencesService().getCurrency().toPrimitives().string
    val coins = withContext(Dispatchers.IO) { widgetService().coins(GemWidgetSize.MEDIUM, currency) }
    return coroutineScope {
        coins.map { coin -> async { coin.uiModel(loadIcon(context, coin.icon)) } }.awaitAll()
    }
}

private fun GemWidgetCoin.uiModel(icon: Bitmap?): WidgetCoinUIModel = WidgetCoinUIModel(
    name = name,
    symbol = symbol,
    priceText = price.text(),
    changeText = change.text(),
    changeStyle = change.tone.textStyle(),
    icon = icon,
)

private suspend fun loadIcon(context: Context, icon: GemAssetIcon): Bitmap? = withContext(Dispatchers.IO) {
    val data: Any = when (val image = icon.image) {
        is GemAssetIconImage.Local -> image.chain.toChain().iconResource() ?: return@withContext null
        is GemAssetIconImage.LocalToken -> image.token.iconResource()
        is GemAssetIconImage.Remote -> image.url
    }
    runCatching {
        val request = ImageRequest.Builder(context).data(data).build()
        (context.imageLoader.execute(request) as? SuccessResult)?.image?.toBitmap()
    }.getOrNull()
}
