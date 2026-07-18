package com.operit.hermes

import android.app.Application

class HermesApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        instance = this
    }

    companion object {
        lateinit var instance: HermesApplication
            private set
    }
}
