import path from 'path';
import fsExtra from 'fs-extra';

import { PakeAppOptions } from '@/types';
import { npmDirectory } from '@/utils/dir';

export async function mergeNativeConfig(url: string, options: PakeAppOptions) {
  const { width, height, name = 'pake-app' } = options;

  const nativeConfigDir = path.join(npmDirectory, 'src-tauri-native', '.pake');
  await fsExtra.ensureDir(nativeConfigDir);

  const config = {
    url,
    app_name: name,
    width,
    height,
  };

  const configPath = path.join(nativeConfigDir, 'native_config.json');
  await fsExtra.outputJSON(configPath, config, { spaces: 2 });
}
