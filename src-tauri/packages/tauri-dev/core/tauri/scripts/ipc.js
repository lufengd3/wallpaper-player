// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * @typedef {{callback: string, error: string, data: *}} IsolationPayload - a valid isolation payload
 */

;
(function () {
  /**
   * @type {string}
   */
  const pattern = window.__TAURI_PATTERN__.pattern

  /**
   * @type {string}
   */
  const isolationOrigin = __TEMPLATE_isolation_origin__

  /**
   * @type {{queue: object[], ready: boolean, frame: HTMLElement | null}}
   */
  const isolation = Object.create(null)
  isolation.queue = []
  isolation.ready = false
  isolation.frame = null

  /**
   * Detects if a message event is a valid isolation message.
   *
   * @param {MessageEvent<object>} event - a message event that is expected to be an isolation message
   * @return {boolean} - if the event was a valid isolation message
   */
  function isIsolationMessage(event) {
    return (
      typeof event.data === 'object' &&
      'nonce' in event.data &&
      'payload' in event.data
    )
  }

  /**
   * Detects if data is able to transform into an isolation payload.
   *
   * @param {object} data - object that is expected to contain at least a callback and error identifier
   * @return {boolean} - if the data is able to transform into an isolation payload
   */
  function isIsolationPayload(data) {
    return typeof data === 'object' && 'callback' in data && 'error' in data
  }

  /**
   * Sends a properly formatted message to the isolation frame.
   *
   * @param {IsolationPayload} data - data that has been validated to be an isolation payload
   */
  function sendIsolationMessage(data) {
    // set the frame dom element if it's not been set before
    if (!isolation.frame) {
      const frame = document.querySelector('iframe#__tauri_isolation__')
      if (frame.src.startsWith(isolationOrigin)) {
        isolation.frame = frame
      } else {
        console.error(
          'Tauri IPC found an isolation iframe, but it had the wrong origin'
        )
      }
    }

    // ensure we have the target to send the message to
    if (!isolation.frame || !isolation.frame.contentWindow) {
      console.error(
        'Tauri "Isolation" Pattern could not find the Isolation iframe window'
      )
      return
    }

    isolation.frame.contentWindow.postMessage(
      data,
      '*' /* todo: set this to the secure origin */
    )
  }

  Object.defineProperty(window, '__TAURI_IPC__', {
    // todo: JSDoc this function
    value: Object.freeze((message) => {
      switch (pattern) {
        case 'brownfield':
          window.__TAURI_POST_MESSAGE__(message)
          break

        case 'isolation':
          if (!isIsolationPayload(message)) {
            console.error(
              'Tauri "Isolation" Pattern found an invalid isolation message payload',
              message
            )
            break
          }

          if (isolation.ready) {
            sendIsolationMessage(message)
          } else {
            isolation.queue.push(message)
          }

          break

        case 'error':
          console.error(
            'Tauri IPC found a Tauri Pattern, but it was an error. Check for other log messages to find the cause.'
          )
          break

        default:
          console.error(
            'Tauri IPC did not find a Tauri Pattern that it understood.'
          )
          break
      }
    })
  })

  /**
   * IMPORTANT: See isolation_secure.js for the isolation frame implementation.
   * main frame -> isolation frame = isolation payload
   * isolation frame -> main frame = isolation message
   */
  if (pattern === 'isolation') {
    window.addEventListener(
      'message',
      (event) => {
        // watch for the isolation frame being ready and flush any queued messages
        if (event.data === '__TAURI_ISOLATION_READY__') {
          isolation.ready = true

          for (const message of isolation.queue) {
            sendIsolationMessage(message)
          }

          isolation.queue = []
          return
        }

        if (isIsolationMessage(event)) {
          window.__TAURI_POST_MESSAGE__(event.data)
        }
      },
      false
    )
  }
})()
