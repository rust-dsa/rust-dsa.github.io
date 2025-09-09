# Writing Guidelines

## Purpose
These guidelines help contributors maintain a consistent, high-quality style throughout the project.

## Audience
All contributors and editors working on documentation or book content.

## Writing Style
- Use clear, simple language.
- Write in the active voice.
- Prefer present tense.
- Explain all acronyms and jargon on first use.

### Writing Style Specifics
#### For headings
1. Each md file must contain a single heading 1 (`#`)
2. Each word in heading 1 must start with a capital letter, except articles (a, an, the), coordinating conjunctions (but, or, nor, for, so, yet), short prepositions (at, by, for, in, of, off, on, out, to, up, via).
   - ✅Building a Node From Scratch
   - ✅To Kill a Mockingbird
3. Every heading after heading 1 i.e. `##`, `###`,... must start with a capital letter, and a noun with a capital letter.
   - ✅Artificial intelligence with Python
   - ❌To Kill a Mockingbird
   - ✅To kill a Mockingbird
4. The heading given to the page must not be entirely dissimilar to the title give in SUMMARY.md
   - ✅SUMMARY.md: The Meaning | Heading: The Meaning of Life
   - ❌SUMMARY.md: The Meaning | Heading: The Three Stooges

#### For diagrams
1. Treat each diagram as an isolated component. the reader should not have to read the surrounding content to understand the diagram.
2. Use white or light color as background
3. Use black or dark color for content
4. Describe the shorthands use in diagram somewhere *in diagram*.
5. Diagram should have a caption with the diagram number
   - The second diagram in chapter 3, subchapter 2 will be: 3.2.2: <caption>.

## Formatting
- Code samples should be in fenced code blocks.
- Use bulleted or numbered lists where appropriate.

## Language
- Use US English.
- Spell-check all content before submission.

## Content Rules
- Provide examples for complex concepts.
- Cite sources with links when referencing external material.

## Contribution Process
- Submit pull requests for all changes.
- All content will be reviewed before merging.

## Examples

**Good:**
"To create a new list, use the `Vec::new()` function."

**Bad:**
"u can make a list with vec new."