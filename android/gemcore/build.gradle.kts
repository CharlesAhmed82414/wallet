plugins {
    id("com.android.library")
    id("kotlinx-serialization")
}

android {
    namespace = "com.wallet.core"
    

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    testFixtures {
        enable = true
    }
    testOptions {
        unitTests {
            isReturnDefaultValues = true
        }
    }
}

dependencies {
    api(project(":gemstone"))
    api(libs.javax.inject)

    api(libs.kotlinx.serialization.json)
    api(libs.kotlinx.coroutines.android)
    compileOnly(libs.compose.runtime.annotation)

    testFixturesApi(libs.junit)
    testFixturesApi(libs.kotlinx.coroutines.test)

    testImplementation(testFixtures(project(":gemcore")))
    testImplementation(libs.junit)
    testImplementation(libs.kotlinx.coroutines.test)
}
