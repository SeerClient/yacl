package dev.isxander.yacl3.platform;


import net.fabricmc.loader.api.FabricLoader;








import net.minecraft.resources.Identifier;
import java.nio.file.Path;

public final class YACLPlatform {
    public static Identifier parseRl(String rl) {
        return Identifier.parse(rl);
    }

    public static Identifier rl(String path) {
        return rl("yet_another_config_lib_v3", path);
    }

    public static Identifier mcRl(String path) {
        return rl("minecraft", path);
    }

    public static Identifier rl(String namespace, String path) {
        return Identifier.fromNamespaceAndPath(namespace, path);
    }

    public static Env getEnvironment() {
        
        return switch (FabricLoader.getInstance().getEnvironmentType()) {
            case CLIENT -> Env.CLIENT;
            case SERVER -> Env.SERVER;
        };
        






    }

    public static Path getConfigDir() {
        
        return FabricLoader.getInstance().getConfigDir();
        


    }

    public static boolean isDevelopmentEnv() {
        
        return FabricLoader.getInstance().isDevelopmentEnvironment();
        


    }
}
