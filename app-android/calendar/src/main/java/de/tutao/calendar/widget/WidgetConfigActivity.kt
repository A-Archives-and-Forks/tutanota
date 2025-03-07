package de.tutao.calendar.widget

import android.app.Activity
import android.appwidget.AppWidgetManager
import android.content.Intent
import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.background
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.safeDrawingPadding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronLeft
import androidx.compose.material3.ButtonColors
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.rememberTopAppBarState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import de.tutao.tutashared.AndroidNativeCryptoFacade
import de.tutao.tutashared.credentials.CredentialsEncryptionFactory
import de.tutao.tutashared.data.AppDatabase
import de.tutao.tutashared.push.toSdkCredentials

class WidgetConfigActivity : AppCompatActivity() {
	private var appWidgetId = AppWidgetManager.INVALID_APPWIDGET_ID

	override fun onCreate(savedInstanceState: Bundle?) {
		super.onCreate(savedInstanceState)
		val db = AppDatabase.getDatabase(this, true)
		val crypto = AndroidNativeCryptoFacade(this)
		val nativeCredentialsFacade = CredentialsEncryptionFactory.create(this, crypto, db)

//		var allCredentials = listOf()
//		nativeCredentialsFacade.loadAll()

		// Retrieve the App Widget ID from the launching intent.
		appWidgetId = intent?.extras?.getInt(
			AppWidgetManager.EXTRA_APPWIDGET_ID,
			AppWidgetManager.INVALID_APPWIDGET_ID
		) ?: AppWidgetManager.INVALID_APPWIDGET_ID

		if (appWidgetId == AppWidgetManager.INVALID_APPWIDGET_ID) {
			finish()
			return
		}

		// Set default result in case the user cancels.
		var resultValue = Intent().putExtra(AppWidgetManager.EXTRA_APPWIDGET_ID, appWidgetId)
		setResult(Activity.RESULT_CANCELED, resultValue)
		// Use Jetpack Compose to build the configuration UI.
		setContent {
			MaterialTheme(
				colorScheme = if (isSystemInDarkTheme()) {
					AppTheme.DarkColors
				} else {
					AppTheme.LightColors
				}
			) {
				WidgetConfig(
					finishAction = {
						finish()
					}
				)
			}
		}
	}
}

/**
 * @param onRegularSelected function that gets executed when regular button is pressed
 * @param onTonalSelected function that gets executed when tonal button is pressed
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun WidgetConfig(
	finishAction: () -> Unit
) {
	var mailAddress by remember { mutableStateOf("cat@tuta.com") }
	var showDropdown by remember { mutableStateOf(false) }
	val scrollBehavior = TopAppBarDefaults.pinnedScrollBehavior(rememberTopAppBarState())

	Scaffold(
		topBar = {
			Row(modifier = Modifier.padding(horizontal = 0.dp), verticalAlignment = Alignment.CenterVertically) {
				IconButton(
					onClick = { finishAction() }, modifier = Modifier
						.width(44.dp)
						.height(44.dp)
				) {
					Icon(Icons.Default.ChevronLeft, "Back")
				}
				Text("Widget Settings", color = MaterialTheme.colorScheme.onSurface, fontWeight = FontWeight.SemiBold)
			}
		},
		modifier = Modifier
			.nestedScroll(scrollBehavior.nestedScrollConnection)
			.fillMaxSize()
			.background(MaterialTheme.colorScheme.background)
			.safeDrawingPadding()
	) { innerPadding ->
		Column(
			modifier = Modifier
				.fillMaxSize()
				.background(MaterialTheme.colorScheme.background)
				.padding(
					vertical = innerPadding.calculateTopPadding().value.coerceAtLeast(8F).dp,
					horizontal = 8.dp
				),
		) {
			Column(
				modifier = Modifier
					.padding(8.dp)
			) {
				Text(
					"Account".uppercase(),
					color = MaterialTheme.colorScheme.onBackground,
					fontWeight = FontWeight.Bold,
					fontSize = 12.sp,
					lineHeight = 12.sp
				)
				TextButton(
					contentPadding = PaddingValues(8.dp),
					onClick = { showDropdown = true },
					shape = RoundedCornerShape(8.dp),
					colors = ButtonColors(
						contentColor = MaterialTheme.colorScheme.secondary,
						containerColor = MaterialTheme.colorScheme.surface,
						disabledContentColor = MaterialTheme.colorScheme.primary,
						disabledContainerColor = MaterialTheme.colorScheme.onPrimary
					),
					modifier = Modifier
						.fillMaxWidth()
				) {
					Text(mailAddress, modifier = Modifier.fillMaxWidth())
				}
				DropdownMenu(
					expanded = showDropdown,
					onDismissRequest = { showDropdown = false },
					modifier = Modifier.background(MaterialTheme.colorScheme.surface)
				) {
					// First section
					DropdownMenuItem(
						text = { Text("cat@tuta.com") },
						onClick = { showDropdown = false },
						modifier = Modifier
							.background(color = MaterialTheme.colorScheme.surface)
					)
					DropdownMenuItem(
						text = { Text("spongebob@tuta.io") },
						onClick = { showDropdown = false },
						modifier = Modifier
							.background(color = MaterialTheme.colorScheme.surface)
					)
					DropdownMenuItem(
						text = { Text("patrikthestar@tutanota.de") },
						onClick = { showDropdown = false },
						modifier = Modifier
							.background(color = MaterialTheme.colorScheme.surface)
					)
				}
			}

			Column(
				modifier = Modifier
					.padding(8.dp)
			) {
				Text(
					"Calendars".uppercase(),
					color = MaterialTheme.colorScheme.onBackground,
					fontWeight = FontWeight.Bold,
					fontSize = 12.sp,
					lineHeight = 12.sp,
					modifier = Modifier.padding(bottom = 4.dp)
				)
				Card(
					colors = CardDefaults.cardColors(
						containerColor = MaterialTheme.colorScheme.surface,
					),
					modifier = Modifier
						.fillMaxWidth(),
				) {
					Text(
						"No Items",
						color = MaterialTheme.colorScheme.onBackground,
						textAlign = TextAlign.Center,
						modifier = Modifier
							.fillMaxWidth()
							.padding(vertical = 32.dp)
					)
				}
			}
		}
	}
}

@Preview(widthDp = 500, heightDp = 500)
@Composable
fun WidgetConfigPreview() {
	MaterialTheme(
		colorScheme = if (isSystemInDarkTheme()) {
			AppTheme.DarkColors
		} else {
			AppTheme.LightColors
		}
	) {
		WidgetConfig({})
	}
}