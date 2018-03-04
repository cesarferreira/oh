#!/usr/bin/env node

'use strict';

const Chalk = require('chalk');
const Utils = require('./utils/utils');
const log = console.log;
const inquirer = require('inquirer');
const fuzzy = require('fuzzy');
const shell = require('shelljs');
const fs = require('fs')

function fuzzySearch(list, textToFind) {
  textToFind = textToFind || '';

  return new Promise(resolve => {
    var fuzzyResult = fuzzy.filter(textToFind, list);
    resolve(fuzzyResult.map(el => el.original));
  });
}

function getDirectories(path) {
  return fs.readdirSync(path).filter(function (file) {
    return fs.statSync(path+'/'+file).isDirectory();
  });
}

function getAppList() {
  // return ['android studio', 'yo mama', 'more apps']
  return getDirectories(`/Applications/`)
}

function showApplicationList(appList) {
  inquirer.registerPrompt('autocomplete', require('inquirer-autocomplete-prompt'));
  inquirer.prompt({
    type: 'autocomplete',
    name: 'app',
    pageSize: 10,
    message: 'What app do you want to openhere?',
    source: function(answersSoFar, input) {
      return fuzzySearch(appList, input)
    }
  }).then(answer => {
    log(answer)
    openApp(answer.app)
  });
}

function openApp(app) {
  shell.exec(`open -a "${app}" .`, {}).stdout;
}

// Main code //
const self = module.exports = {
  init: (input, flags) => {
      const appList = getAppList()
      showApplicationList(appList);
  }
};
