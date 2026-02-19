package com.sevenwiki.sevenmark

import com.intellij.openapi.fileTypes.LanguageFileType
import javax.swing.Icon

class SevenMarkFileType private constructor() : LanguageFileType(SevenMarkLanguage.INSTANCE) {
    companion object {
        @JvmStatic
        val INSTANCE = SevenMarkFileType()
    }

    override fun getName(): String = "SevenMark"

    override fun getDescription(): String = "SevenMark markup file"

    override fun getDefaultExtension(): String = "sm"

    override fun getIcon(): Icon = SevenMarkIcons.FILE
}