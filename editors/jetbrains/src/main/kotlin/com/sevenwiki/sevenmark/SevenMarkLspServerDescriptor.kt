package com.sevenwiki.sevenmark

import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.ide.plugins.PluginManagerCore
import com.intellij.openapi.extensions.PluginId
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor

class SevenMarkLspServerDescriptor(project: Project) :
    ProjectWideLspServerDescriptor(project, "SevenMark") {

    override fun isSupportedFile(file: VirtualFile): Boolean =
        file.extension == "sm"

    override fun createCommandLine(): GeneralCommandLine {
        val serverPath = findServerBinary()
        return GeneralCommandLine(serverPath)
    }

    private fun findServerBinary(): String {
        val isWindows = System.getProperty("os.name").lowercase().contains("win")
        val binaryName = if (isWindows) "sevenmark_language_server.exe" else "sevenmark_language_server"

        // 1. Look for bundled binary inside the plugin directory
        val plugin = PluginManagerCore.getPlugin(PluginId.getId("com.sevenwiki.sevenmark"))
        if (plugin != null) {
            val bundled = plugin.pluginPath.resolve("server").resolve(binaryName)
            if (bundled.toFile().exists()) {
                return bundled.toString()
            }
        }

        // 2. Fall back to PATH
        return "sevenmark_language_server"
    }
}