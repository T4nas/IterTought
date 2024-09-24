# Iteration of Thought: Implementation

This project implements the concepts presented in the paper "Iteration of Thought: Leveraging Inner Dialogue for Autonomous Large Language Model Reasoning," released at the University of Toronto. You can find the paper [here](https://arxiv.org/pdf/2409.12618).

## Overview

The Iteration of Thought (IoT) method enhances large language model (LLM) responses through iterative human engagement. It utilizes dynamic prompting based on the input query and the current response, allowing LLMs to generate more thoughtful and accurate outputs.

## Key Components

1. **Inner Dialogue Agent (IDA)**: Generates context-specific prompts.
2. **LLM Agent (LLMA)**: Processes prompts to refine responses.
3. **Iterative Prompting Loop**: Facilitates conversation between IDA and LLMA.

## Variants

- **Autonomous Iteration of Thought (AIoT)**: The LLM decides when to stop iterating.
- **Guided Iteration of Thought (GIoT)**: A fixed number of iterations is enforced.

This implementation shows improved performance over traditional approaches, minimizing human intervention and fostering adaptive reasoning in LLMs.
