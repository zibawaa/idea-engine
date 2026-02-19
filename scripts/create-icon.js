const sharp = require('sharp');
const fs = require('fs');
const path = require('path');

const iconsDir = path.join(__dirname, '../apps/desktop/src-tauri/icons');
fs.mkdirSync(iconsDir, { recursive: true });

// Create 32x32 blue square PNG
sharp({
  create: {
    width: 32,
    height: 32,
    channels: 4,
    background: { r: 59, g: 130, b: 246, alpha: 1 }
  }
})
  .png()
  .toFile(path.join(iconsDir, '32x32.png'))
  .then(() => {
    console.log('Created 32x32.png');
    // Create 1024x1024 for tauri icon (recommended size)
    return sharp({
      create: {
        width: 1024,
        height: 1024,
        channels: 4,
        background: { r: 59, g: 130, b: 246, alpha: 1 }
      }
    })
      .png()
      .toFile(path.join(iconsDir, 'app-icon.png'));
  })
  .then(() => {
    console.log('Created app-icon.png - run: pnpm tauri icon apps/desktop/src-tauri/icons/app-icon.png');
  })
  .catch(err => {
    console.error(err);
    process.exit(1);
  });
