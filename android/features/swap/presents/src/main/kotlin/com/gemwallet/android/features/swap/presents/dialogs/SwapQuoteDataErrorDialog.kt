package com.gemwallet.android.features.swap.presents.dialogs

import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.stringResource
import com.gemwallet.android.ui.R
import com.gemwallet.android.ui.localization.text
import uniffi.gemstone.GemSwapErrorDisplay

@Composable
internal fun SwapQuoteDataErrorDialog(error: GemSwapErrorDisplay?, onDismiss: () -> Unit) {
    error ?: return

    AlertDialog(
        onDismissRequest = onDismiss,
        confirmButton = {
            Button(onDismiss) {
                Text(stringResource(R.string.common_done))
            }
        },
        title = { Text(stringResource(R.string.errors_error_occurred)) },
        text = { Text(error.text(LocalContext.current)) },
        containerColor = MaterialTheme.colorScheme.background,
    )
}
