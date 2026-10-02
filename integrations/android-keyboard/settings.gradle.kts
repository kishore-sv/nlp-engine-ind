pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}
rootProject.name = "nlp-engine-ind-android-keyboard"
include(":app")
include(":nlp-engine-ind")
project(":nlp-engine-ind").projectDir = file("../../bindings/android/nlp-engine-ind")
