#!/usr/bin/env node

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

function cleanDistForBuild() {
  const buildDir = path.resolve(__dirname + "./../dist");
  const buildDirExists = fs.existsSync(buildDir);

  if (buildDirExists) {
    fs.rmSync(buildDir, { maxRetries: 3, force: true, recursive: true });
  }
}

cleanDistForBuild();
