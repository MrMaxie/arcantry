#!/usr/bin/env node
import { runCli } from './program.js';

const result = await runCli();
process.exitCode = result.exitCode;
