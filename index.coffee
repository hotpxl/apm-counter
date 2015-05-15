#!/usr/bin/env coffee
childProcess = require 'child_process'
util = require 'util'
moment = require 'moment'
_ = require 'lodash'

decay = 0.2
interval = 0.1
device = 10

currentCount = 0
averageSpeed = 0

keyLogger = do ->
  keyPressed = false
  ->
    keyPressed = !keyPressed
    if keyPressed
      currentCount += 1

keyPrinter = ->
  averageSpeed = decay * currentCount + (1 - decay) * averageSpeed
  currentCount = 0
  speed = averageSpeed * 60 / interval
  maxSpeed = 1000
  process.stdout.write '\x1b[2K\r['
  total = 50
  progress = Math.min Math.floor(speed / maxSpeed * total), total
  _.map _.range(progress), ->
    process.stdout.write '>'
  _.map _.range(total - progress), ->
    process.stdout.write '-'
  speed = speed.toFixed(2).toString()
  while speed.length < 7
    speed = ' ' + speed
  process.stdout.write "] #{speed} APM"
  if 0.8 * total <= progress
    process.stdout.write '  \x1b[1;31m>>> WARNING <<<\x1b[0m'

child = childProcess.spawn 'xinput', ['test', device]
child.stdout.on 'data', (str) ->
  _.map str.toString(), (i) ->
    if i == '\n'
      keyLogger()

setInterval keyPrinter, interval * 1000

process.on 'exit', ->
  child.kill()
