# Manus AI Toolset Reference

This document provides an exhaustive list of all tools available to Manus AI, including their original descriptions and detailed parameters.

## 1. Planning & Communication

### `plan`
*   **Description**: Create, update, and advance the structured task plan.
*   **Parameters**:
    *   `action`: The action to perform (`update`, `advance`).
    *   `current_phase_id`: ID of the phase the task is currently in.
    *   `goal`: The overall goal of the task (required for `update`).
    *   `next_phase_id`: ID of the phase the task is advancing to (required for `advance`).
    *   `phases`: Complete list of phases required to achieve the task goal (required for `update`).

### `message`
*   **Description**: Send messages to interact with the user.
*   **Parameters**:
    *   `type`: The type of the message (`info`, `ask`, `result`).
    *   `text`: The message or question text to be shown to the user.
    *   `attachments`: A list of attachments to include with the message.
    *   `suggested_action`: The suggested action for the user to take (`none`, `confirm_browser_operation`, `take_over_browser`, `upgrade_to_unlock_feature`).

## 2. Web & Information Gathering

### `search`
*   **Description**: Search for information across various sources.
*   **Parameters**:
    *   `type`: The category of search to perform (`info`, `image`, `api`, `news`, `tool`, `data`, `research`).
    *   `queries`: Up to 3 query variants that express the same search intent.
    *   `time`: Optional time filter to limit results to a recent time range (`all`, `past_day`, `past_week`, `past_month`, `past_year`).
    *   `brief`: A one-sentence preamble describing the purpose of this operation.

### `browser`
*   **Description**: Navigate the browser to a specified URL to begin web browsing session.
*   **Parameters**:
    *   `url`: The URL to navigate to.
    *   `intent`: The purpose of visiting this URL (`navigational`, `informational`, `transactional`).
    *   `focus`: Specific topic, section, or question to focus on when visiting the page (required if intent is `informational`).
    *   `brief`: A one-sentence preamble describing the purpose of this operation.

## 3. File & System Management

### `file`
*   **Description**: Perform operations on files in the sandbox file system.
*   **Parameters**:
    *   `action`: The action to perform (`view`, `read`, `write`, `append`, `edit`).
    *   `path`: The absolute path to the target file.
    *   `edits`: A list of edits to be sequentially applied to the file (required for `edit`).
    *   `range`: An array of two integers specifying the start and end of the range (optional for `view` and `read`).
    *   `text`: The content to be written or appended (required for `write` and `append`).
    *   `brief`: A one-sentence preamble describing the purpose of this operation.

### `match`
*   **Description**: Find files or text in the sandbox file system using pattern matching.
*   **Parameters**:
    *   `action`: The action to perform (`glob`, `grep`).
    *   `scope`: The glob pattern that defines the absolute file path and name scope.
    *   `leading`: Number of lines to include before each match as context (optional for `grep`).
    *   `regex`: The regex pattern to match file content (required for `grep`).
    *   `trailing`: Number of lines to include after each match as context (optional for `grep`).
    *   `brief`: A one-sentence preamble describing the purpose of this operation.

### `shell`
*   **Description**: Interact with shell sessions in the sandbox environment.
*   **Parameters**:
    *   `brief`: A one-sentence preamble describing the purpose of this operation.
    *   `action`: The action to perform (`view`, `exec`, `wait`, `send`, `kill`).
    *   `session`: The unique identifier of the target shell session.
    *   `command`: The shell command to execute (required for `exec`).
    *   `input`: Input text to send to the active process (stdin) (required for `send`).
    *   `timeout`: Timeout in seconds to wait for command execution (optional for `exec` and `wait`).

### `expose`
*   **Description**: Expose a local port in the sandbox for temporary public access.
*   **Parameters**:
    *   `brief`: A one-sentence preamble describing the purpose of this operation.
    *   `port`: Local port number in the sandbox to expose for public access.

## 4. Creative & Generative Tools

### `generate`
*   **Description**: Enter generation mode to create or edit images, videos, audio, and speech from text and media references.
*   **Parameters**:
    *   `brief`: A one-sentence preamble describing the purpose of this operation.

### `slides`
*   **Description**: Enter slides mode to handle presentation creation and adjustment.
*   **Parameters**:
    *   `brief`: A one-sentence preamble describing the purpose of this operation.
    *   `slide_content_file_path`: Path to markdown file in sandbox containing the detailed slide content outline.
    *   `slide_count`: Total number of slides in the presentation.
    *   `generate_mode`: The generation mode that determines how slides are rendered and output (`html`, `image`).

## 5. Development & Automation

### `webdev_init_project`
*   **Description**: Initialize a new web or mobile app project with scaffold and automated environment setup.
*   **Parameters**:
    *   `brief`: A one-sentence description of the project initialization purpose.
    *   `name`: Name of the web project to be created.
    *   `title`: Title of the web project to be created.
    *   `description`: Description of the web project to be created.
    *   `scaffold`: Project scaffold type (`web-static`, `web-db-user`, `mobile-app`).

### `schedule`
*   **Description**: Schedule a task to run at a specific time or interval.
*   **Parameters**:
    *   `brief`: A one-sentence preamble describing the purpose of this operation.
    *   `type`: Type of schedule for the task (`cron`, `interval`).
    *   `repeat`: Whether to repeat the task after execution.
    *   `name`: Concise human-readable name of the task.
    *   `prompt`: Natural language description of the task to perform at execution time.
    *   `cron`: Standard 6-field cron expression (required for `cron` type).
    *   `expire`: Optional datetime string (yyyy-MM-dd HH:mm:ss) specifying when the task should expire.
    *   `interval`: Time interval in seconds between executions (required for `interval` type).
    *   `playbook`: Summary of process and best practices learned from the current task.

## 6. Pre-installed Shell Utilities
These utilities are invoked via the `shell` tool with specific command-line arguments.

### `manus-render-diagram`
*   **Description**: Render diagram files (.mmd, .d2, .puml, .md) to PNG format.
*   **Usage**: `$ manus-render-diagram <input_file> <output_file>`

### `manus-md-to-pdf`
*   **Description**: Convert Markdown file to PDF format.
*   **Usage**: `$ manus-md-to-pdf <input_file> <output_file>`

### `manus-speech-to-text`
*   **Description**: Transcribe speech/audio files (.mp3, .wav, .mp4, .webm) to text.
*   **Usage**: `$ manus-speech-to-text <input_file>`

### `manus-mcp-cli`
*   **Description**: Interact with Model Context Protocol (MCP) servers.
*   **Usage**: `$ manus-mcp-cli <command> [args...]`

### `manus-upload-file`
*   **Description**: Upload one or more files to S3 and get direct public URLs for MCP or API invocations.
*   **Usage**: `$ manus-upload-file <input_file> [input_file_2 ...]`

### `manus-export-slides`
*   **Description**: Export slides from manus-slides://{version_id} URI to specified format (.pdf, .ppt).
*   **Usage**: `$ manus-export-slides manus-slides://2tvrCaJBV8I6gabDLa4YCL pdf`