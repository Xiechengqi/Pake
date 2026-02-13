import path from 'path';
import fsExtra from 'fs-extra';
import chalk from 'chalk';
import prompts from 'prompts';

import { PakeAppOptions } from '@/types';
import { checkRustInstalled, ensureRustEnv, installRust } from '@/helpers/rust';
import { mergeNativeConfig } from '@/helpers/native-merge';
import { npmDirectory } from '@/utils/dir';
import { getSpinner } from '@/utils/info';
import { shellExec } from '@/utils/shell';
import { IS_MAC, IS_WIN } from '@/utils/platform';
import logger from '@/options/logger';

export default class NativeBrowserBuilder {
  private options: PakeAppOptions;

  constructor(options: PakeAppOptions) {
    this.options = options;
  }

  async prepare() {
    ensureRustEnv();

    if (!checkRustInstalled()) {
      const res = await prompts({
        type: 'confirm',
        message: 'Rust not detected. Install now?',
        name: 'value',
      });

      if (res.value) {
        await installRust();
      } else {
        logger.error('✕ Rust required to build native browser app.');
        process.exit(0);
      }
    }
  }

  async start(url: string) {
    const { name = 'pake-app' } = this.options;

    await mergeNativeConfig(url, this.options);

    const manifestPath = path.join(
      npmDirectory,
      'src-tauri-native',
      'Cargo.toml',
    );

    logger.info('Building and running native browser app in dev mode...');
    const runCommand = `cargo run --manifest-path "${manifestPath}"`;
    await shellExec(runCommand, 600000);
  }

  async build(url: string) {
    const { name = 'pake-app' } = this.options;

    await mergeNativeConfig(url, this.options);

    const manifestPath = path.join(
      npmDirectory,
      'src-tauri-native',
      'Cargo.toml',
    );

    const buildSpinner = getSpinner('Building native browser app...');
    await new Promise((resolve) => setTimeout(resolve, 500));
    buildSpinner.stop();
    logger.warn('✸ Building native browser app...');

    const buildCommand = `cargo build --release --manifest-path "${manifestPath}"`;
    await shellExec(buildCommand, 600000);

    const targetDir = path.join(
      npmDirectory,
      'src-tauri-native',
      'target',
      'release',
    );
    const binaryName = IS_WIN ? 'pake-native.exe' : 'pake-native';
    const sourceBinary = path.join(targetDir, binaryName);

    if (IS_MAC) {
      await this.createMacAppBundle(sourceBinary, name);
    } else {
      const ext = IS_WIN ? '.exe' : '';
      const outputPath = path.resolve(`${name}${ext}`);
      await fsExtra.copy(sourceBinary, outputPath);
      if (!IS_WIN) {
        await fsExtra.chmod(outputPath, 0o755);
      }
      logger.success('✔ Build success!');
      logger.success('✔ App binary located in', outputPath);
    }
  }

  private async createMacAppBundle(sourceBinary: string, name: string) {
    const appDir = path.resolve(`${name}.app`);
    const contentsDir = path.join(appDir, 'Contents');
    const macosDir = path.join(contentsDir, 'MacOS');
    const resourcesDir = path.join(contentsDir, 'Resources');

    await fsExtra.ensureDir(macosDir);
    await fsExtra.ensureDir(resourcesDir);

    // Copy binary
    const binaryDest = path.join(macosDir, name);
    await fsExtra.copy(sourceBinary, binaryDest);
    await fsExtra.chmod(binaryDest, 0o755);

    // Generate Info.plist
    const plist = `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>${name}</string>
  <key>CFBundleIdentifier</key>
  <string>${this.options.identifier}</string>
  <key>CFBundleName</key>
  <string>${name}</string>
  <key>CFBundleVersion</key>
  <string>${this.options.appVersion}</string>
  <key>CFBundleShortVersionString</key>
  <string>${this.options.appVersion}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>`;
    await fsExtra.writeFile(path.join(contentsDir, 'Info.plist'), plist);

    // Copy icon if provided
    if (this.options.icon) {
      const iconPath = path.resolve(this.options.icon);
      if (
        (await fsExtra.pathExists(iconPath)) &&
        iconPath.endsWith('.icns')
      ) {
        await fsExtra.copy(iconPath, path.join(resourcesDir, 'icon.icns'));
      }
    }

    logger.success('✔ Build success!');
    logger.success('✔ App bundle located in', appDir);
  }
}
