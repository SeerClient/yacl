package dev.isxander.yacl3.platform;

import dev.isxander.yacl3.gui.image.YACLImageReloadListener;


import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.resource.v1.ResourceLoader;
import net.minecraft.server.packs.PackType;

public class PlatformEntrypoint implements ClientModInitializer {
    @Override
    public void onInitializeClient() {
        YACLConfig.HANDLER.load();
        ResourceLoader.get(PackType.CLIENT_RESOURCES).registerReloadListener(YACLImageReloadListener.getId(), new YACLImageReloadListener());
    }
}






























