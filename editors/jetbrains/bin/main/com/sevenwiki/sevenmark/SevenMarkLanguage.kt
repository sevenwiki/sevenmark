package com.sevenwiki.sevenmark

import com.intellij.lang.Language

class SevenMarkLanguage private constructor() : Language("SevenMark") {
    companion object {
        @JvmStatic
        val INSTANCE = SevenMarkLanguage()
    }
}
