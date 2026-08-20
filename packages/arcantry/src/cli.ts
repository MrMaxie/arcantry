#!/usr/bin/env node
import { runNativeCli } from './nativeLauncher.js';

process.exitCode = runNativeCli(process.argv.slice(2));
