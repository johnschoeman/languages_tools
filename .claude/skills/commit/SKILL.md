---
name: commit
description: Review git state and create a commit message following repository conventions. Suggests a message by default; pass --run to commit directly. Use when the user wants to commit, says "commit", or asks for a commit message.
argument-hint: [--run]
---

# Commit

Writes a commit message to `commit-msg.txt`. Pass `--run` to commit directly.

## Process

### 1. Gather Git State

Run in parallel:

```bash
git status
git diff --staged
git diff
git log --oneline -5
```

If no changes are staged or modified, inform the user and stop.

### 2. Analyze Changes

- What files are modified, added, or deleted?
- What is the nature of the changes? (new feature, fix, refactor, docs, etc.)
- Why were these changes made? (infer from context)
- Exclude `commit-msg.txt` from consideration — never include it in a commit.

### 3. Write Commit Message

Copy `TEMPLATE.md` from this skill directory and fill in the placeholders.

No prefix — plain imperative title only (no `feat:`, no ticket IDs).

**Title:** imperative mood, no period, capitalize first letter. **ALWAYS stay within 50 characters — no exceptions.** Count the characters before writing.

**Body:** explain WHY not just WHAT, bullet points for multiple changes. **ALWAYS hard-wrap every line at 72 characters — no exceptions.** This includes bullet points, prose, and the "Why:" section.

Always include the full body (Why + This commit) regardless of change size.

### 4. Write to File

Write the commit message to `commit-msg.txt` in the project root.

### 5. Deliver

Show the commit message to the user.

- **Default (no flag):** Tell the user to review `commit-msg.txt` and run `git commit -F commit-msg.txt` when ready.
- **With `--run`:** Run `git add -p` if nothing is staged, then `git commit -F commit-msg.txt`. Show the result.

## Notes

- For trivial changes (typos, minor tweaks), keep the body concise but still include Why and This commit sections.
- Don't include file lists in commit messages unless they add clarity.
- Never commit `commit-msg.txt` itself.

<!-- catalog: commit v1 -->
