package com.gemwallet.android.ui.components.empty

import androidx.annotation.DrawableRes

sealed interface EmptyStateImage {
    @JvmInline value class Drawable(@param:DrawableRes val id: Int) : EmptyStateImage

    @JvmInline value class Vector(@param:DrawableRes val id: Int) : EmptyStateImage
}
