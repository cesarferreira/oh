#!/usr/bin/env node

'use strict';

const meow = require('meow');
const app = require('./src/app');
const updateNotifier = require('update-notifier');
const pkg = require('./package.json');

updateNotifier({
    pkg
}).notify();

const cli = meow(`
Usage
   $ oh
`, {
    alias: {
        v: 'version'
    },
    boolean: ['version']
});

app.init(cli.input, cli.flags);