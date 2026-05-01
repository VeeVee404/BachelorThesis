from pathlib import Path
from xml.sax.saxutils import escape
import zipfile


ROWS = [
    [
        "Day1_Solution_2025.py",
        4,
        "The control flow is easy to follow and the variable names are clear, but it is still a compact script with minimal explanation.",
        3,
        "Using main helps, but the parsing and movement logic are all kept inline with no further decomposition.",
        4,
        "The file iteration, string parsing, and modular arithmetic are standard Python choices for this kind of puzzle script.",
        5,
        "The implementation is lean and direct, with no unnecessary abstraction beyond optional timing code.",
    ],
    [
        "Day1b_Solution_2025.py",
        4,
        "The helper name is good and the flow is clear, although the first-hit arithmetic still takes a moment to verify.",
        4,
        "The counting logic is cleanly extracted into a helper, which gives the script a clearer shape than a single monolithic main.",
        4,
        "The type hints, helper extraction, and modular arithmetic fit normal Python practice well.",
        4,
        "The arithmetic shortcut adds some density, but it is still a focused solution without much extra machinery.",
    ],
    [
        "Day2_Solution_2025.py",
        3,
        "The helper boundaries help, but the mathematical intent behind names like base, first, and last is not immediately obvious.",
        4,
        "The script is sensibly split into small helpers for ceiling division, range sums, and the main range calculation.",
        4,
        "Integer arithmetic helpers and straightforward file parsing are used in a solid Python style.",
        3,
        "The number-pattern math is compact and efficient, but it is denser than necessary for quick comprehension.",
    ],
    [
        "Day2b_Solution_2025.py",
        4,
        "The helper names are descriptive and the data flow is readable, though the nested generation logic is still somewhat heavy to scan.",
        4,
        "Precomputation, prefix-sum building, and range queries are separated into focused helpers with a clear main routine.",
        4,
        "Using set accumulation, sorting, prefix sums, and bisect is a good Python fit for this approach.",
        3,
        "The precompute-and-query design is workable, but it introduces more machinery than the simpler part-one style.",
    ],
    [
        "Day6_Solution_2025.py",
        4,
        "The phases of padding, separator detection, and evaluation are easy to follow, with sensible variable names throughout.",
        3,
        "The code has a clear top-level flow, but all parsing and evaluation still live inline inside one function.",
        4,
        "List comprehensions, all, and reduce with mul are conventional Python tools for this task.",
        4,
        "The bookkeeping for padded columns is problem-driven and mostly restrained, with only minor extra setup.",
    ],
    [
        "Day6b_Solution_2025.py",
        3,
        "The overall flow is understandable, but reconstructing values column by column in reverse order is noticeably harder to read.",
        4,
        "Extracting parse_problem gives the solution a cleaner shape and isolates the trickiest part of the logic.",
        4,
        "The loops, list building, and explicit multiplication are ordinary Python and fit the problem well.",
        3,
        "The reversed-column reconstruction adds real cognitive overhead even if the implementation itself stays fairly small.",
    ],
    [
        "Day7_Solution_2025.py",
        4,
        "The state transition is short and easy to trace, and the names make the branching behavior clear.",
        3,
        "Everything sits inside main, so the script is organized but not meaningfully decomposed.",
        4,
        "Set-based state tracking and direct grid traversal are idiomatic Python choices here.",
        5,
        "The solution is minimal and stays tightly focused on the required behavior.",
    ],
    [
        "Day7b_Solution_2025.py",
        4,
        "The count propagation is clear and the fallback input loader is easy to understand, though slightly more verbose than part one.",
        4,
        "Separating file loading from the dynamic-programming loop gives the script a clean and sensible structure.",
        4,
        "The use of enumerate, list-based counts, and exception handling is standard Python practice.",
        4,
        "The extra input-file fallback and path-count state add some weight, but the design remains controlled.",
    ],
    [
        "Day11_Solution_2025.py",
        4,
        "The recursive path count is concise and easy to read, with very little noise around the core idea.",
        4,
        "Parsing and memoized evaluation are separated cleanly, even though the cached helper remains nested inside main.",
        5,
        "Using lru_cache over a recursive graph traversal is a very natural Python solution for this problem.",
        5,
        "The implementation is compact and avoids unnecessary machinery entirely.",
    ],
    [
        "Day11b_Solution_2025.py",
        4,
        "The bitmask constants make the intent readable, although the state-tracking logic is denser than the simpler first part.",
        4,
        "The graph parsing and cached search are still organized cleanly, with the extra state folded into a focused helper.",
        5,
        "Bitmasks plus lru_cache are an idiomatic and efficient Python way to encode this extra path state.",
        4,
        "The added complexity comes from the problem requirements and is handled without much extra overhead.",
    ],
    [
        "Day12_Solution_2025.py",
        2,
        "The duplicated timing code, index-heavy parsing, and mixed phases make the script noticeably harder to scan than the others.",
        2,
        "Most of the logic is packed into one long main function, and there is also redundant script-level execution code around it.",
        3,
        "Basic file reading and list processing are normal Python, but the duplicated top-level timing and manual parser structure are not especially clean.",
        2,
        "The duplicate runtime measurement and tightly coupled index manipulation add avoidable complexity on top of a simple area check.",
    ],
]


def col_name(index: int) -> str:
    result = ""
    while index > 0:
        index, rem = divmod(index - 1, 26)
        result = chr(65 + rem) + result
    return result


def cell_xml(col: int, row: int, value: object) -> str:
    ref = f"{col_name(col)}{row}"
    if isinstance(value, int):
        return f'<c r="{ref}"><v>{value}</v></c>'
    return f'<c r="{ref}" t="inlineStr"><is><t>{escape(str(value))}</t></is></c>'


def build_sheet_xml(rows: list[list[object]]) -> str:
    xml_rows = []
    for row_idx, row in enumerate(rows, start=1):
        cells = "".join(cell_xml(col_idx, row_idx, value) for col_idx, value in enumerate(row, start=1))
        xml_rows.append(f'<row r="{row_idx}">{cells}</row>')
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">'
        f"<sheetData>{''.join(xml_rows)}</sheetData>"
        "</worksheet>"
    )


def main() -> None:
    header = [
        "File",
        "Readability",
        "Readability Justification",
        "Structure",
        "Structure Justification",
        "Idiomatic Use",
        "Idiomatic Justification",
        "Unnecessary Complexity",
        "Complexity Justification",
    ]
    rows = [header, *ROWS]

    output = Path("LLM-as-a-judge_2025.xlsx")
    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED) as zf:
        zf.writestr(
            "[Content_Types].xml",
            """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
</Types>
""",
        )
        zf.writestr(
            "_rels/.rels",
            """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>
""",
        )
        zf.writestr(
            "xl/workbook.xml",
            """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets>
    <sheet name="Scores" sheetId="1" r:id="rId1"/>
  </sheets>
</workbook>
""",
        )
        zf.writestr(
            "xl/_rels/workbook.xml.rels",
            """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
</Relationships>
""",
        )
        zf.writestr("xl/worksheets/sheet1.xml", build_sheet_xml(rows))


if __name__ == "__main__":
    main()
