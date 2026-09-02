package app.coomi;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

public class CoomiBackupArchiveTest {
    @Test
    public void skipsRootfsVirtualDescriptors() {
        assertTrue(CoomiBackupArchive.shouldSkipBackupEntry(
            "coomi-home/runtime-v2/versions/debian/rootfs", "dev"));
        assertTrue(CoomiBackupArchive.shouldSkipBackupEntry(
            "coomi-home/runtime-v2/versions/debian/rootfs/dev", "fd"));
        assertTrue(CoomiBackupArchive.shouldSkipBackupEntry(
            "coomi-home/runtime-v2/versions/debian/rootfs", "proc"));
        assertTrue(CoomiBackupArchive.shouldSkipBackupEntry(
            "coomi-home/runtime-v2/versions/debian/rootfs", "sys"));
    }

    @Test
    public void skipsRuntimeEnvironmentTrees() {
        assertTrue(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home/runtime-v2", "versions"));
        assertTrue(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home/runtime-v2", "downloads"));
        assertTrue(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home/runtime-v2", "tmp"));
        assertTrue(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home", "cache"));
        assertTrue(CoomiBackupArchive.isRuntimeEnvironmentPath(
            "coomi-home/runtime-v2/versions/debian/rootfs/usr/bin/bash"));
    }

    @Test
    public void keepsUserDataAndGuestHome() {
        assertFalse(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home", "config"));
        assertFalse(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home", "sessions"));
        assertFalse(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home", "skills"));
        assertFalse(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home/runtime-v2", "home"));
        assertFalse(CoomiBackupArchive.shouldSkipBackupEntry("coomi-home/runtime-v2", "state.json"));
        assertFalse(CoomiBackupArchive.shouldSkipBackupEntry("user-data", "projects"));
        assertFalse(CoomiBackupArchive.shouldSkipBackupEntry("user-data/projects", "dev"));
    }
}
