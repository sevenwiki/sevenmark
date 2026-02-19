package com.sevenwiki.sevenmark

import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor
import java.nio.file.Path

class SevenMarkLspServerDescriptor(project: Project) :
    ProjectWideLspServerDescriptor(project, "SevenMark") {

    override fun isSupportedFile(file: VirtualFile): Boolean =
        file.extension == "sm"

    override fun createCommandLine(): GeneralCommandLine {
        val serverPath = findServerBinary()
        return GeneralCommandLine(serverPath).apply {
            withEnvironment("RUST_LOG", "debug")
        }
    }

    private fun findServerBinary(): String {
        // 1. Look for bundled binary inside the plugin directory
        val pluginDir = Path.of(javaClass.protectionDomain.codeSource.location.toURI()).parent
        val isWindows = System.getProperty("os.name").lowercase().contains("win")
        val binaryName = if (isWindows) "sevenmark_language_server.exe" else "sevenmark_language_server"

        val bundled = pluginDir.resolve("server").resolve(binaryName)
        if (bundled.toFile().exists()) {
            return bundled.toString()
        }

        // 2. Fall back to PATH
        return "sevenmark_language_server"
    }
}
