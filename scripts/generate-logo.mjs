#!/usr/bin/env node
/**
 * 生成水獭 Logo PNG 图片
 * 使用 puppeteer 将 emoji 渲染为图片
 */

import puppeteer from 'puppeteer';
import { writeFileSync, mkdirSync, existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const outputDir = join(__dirname, '../client/packages/app/src/static/logo');

// 需要生成的尺寸
const sizes = [
  { name: 'logo-1024', size: 1024 },  // App Store
  { name: 'logo-512', size: 512 },    // Android
  { name: 'logo-256', size: 256 },    // 启动页
  { name: 'logo-128', size: 128 },    // 通用
  { name: 'logo-96', size: 96 },      // TabBar 2x
  { name: 'logo-48', size: 48 },      // TabBar 1x
];

async function generateLogo() {
  // 确保输出目录存在
  if (!existsSync(outputDir)) {
    mkdirSync(outputDir, { recursive: true });
  }

  console.log('启动浏览器...');
  const browser = await puppeteer.launch({
    headless: true,
    args: ['--no-sandbox']
  });

  const page = await browser.newPage();

  for (const { name, size } of sizes) {
    console.log(`生成 ${name}.png (${size}x${size})...`);

    // 设置页面大小
    await page.setViewport({ width: size, height: size, deviceScaleFactor: 1 });

    // 渲染 emoji
    await page.setContent(`
      <!DOCTYPE html>
      <html>
      <head>
        <style>
          * { margin: 0; padding: 0; box-sizing: border-box; }
          body {
            width: ${size}px;
            height: ${size}px;
            display: flex;
            align-items: center;
            justify-content: center;
            background: transparent;
          }
          .emoji {
            font-size: ${size * 0.8}px;
            line-height: 1;
            font-family: "Apple Color Emoji", "Segoe UI Emoji", "Noto Color Emoji", sans-serif;
          }
        </style>
      </head>
      <body>
        <span class="emoji">🦦</span>
      </body>
      </html>
    `);

    // 截图保存
    const buffer = await page.screenshot({
      type: 'png',
      omitBackground: true,  // 透明背景
    });

    const outputPath = join(outputDir, `${name}.png`);
    writeFileSync(outputPath, buffer);
    console.log(`  -> ${outputPath}`);
  }

  await browser.close();
  console.log('\n完成! 所有 Logo 已生成到:', outputDir);
}

generateLogo().catch(console.error);
