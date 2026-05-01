from pathlib import Path
from xml.sax.saxutils import escape
import zipfile


ROWS = [
    [
        "Day1_Solution_2024.py",
        4,
        "The data flow is easy to follow and the names are clear, but the timing boilerplate and script-level execution keep it short of excellent.",
        2,
        "Input parsing, sorting, scoring, and timing all live at module scope with no functional decomposition.",
        4,
        "List building, sorting, zip, and accumulation are standard Python choices for this task.",
        5,
        "The solution is lean and direct, with no extra machinery beyond minor timing code.",
    ],
    [
        "Day1b_Solution_2024.py",
        4,
        "The counting idea is clear and the variable names are straightforward, though the full workflow is still compacted into one function.",
        3,
        "Wrapping the logic in main is cleaner than raw script code, but there is still no decomposition beyond that.",
        4,
        "Dictionary counting with get and a generator-based sum is an idiomatic Python fit.",
        5,
        "It stays tightly focused on the task without introducing unnecessary abstraction.",
    ],
    [
        "Day2_Solution_2024.py",
        4,
        "The rule-checking helper reads clearly, although the repeated direction checks add a little friction.",
        4,
        "Extracting the safety test into a dedicated helper gives the script a clear top-level shape.",
        4,
        "The loop-based validation is conventional Python and fits the problem well.",
        4,
        "The approach is straightforward, with only mild repetition in the condition checks.",
    ],
    [
        "Day2b_Solution_2024.py",
        4,
        "The intent remains easy to follow because the helper names are strong and the control flow is simple.",
        4,
        "The logic is cleanly split into validation, dampener handling, and orchestration.",
        4,
        "Type hints, slicing, and focused helpers are used in a solid Python style.",
        3,
        "Rechecking every one-element removal is simple but somewhat heavier and more repetitive than ideal.",
    ],
    [
        "Day12_Solution_2024.py",
        4,
        "The flood-fill is readable and the names are good, though the nested traversal logic still takes some scanning.",
        3,
        "The phases are sensible, but parsing and traversal are both embedded directly in script-level code.",
        4,
        "Deque-based BFS and straightforward grid bookkeeping are idiomatic Python choices here.",
        4,
        "The implementation is direct, with only the bookkeeping needed for area and perimeter counting.",
    ],
    [
        "Day12b_Solution_2024.py",
        3,
        "The overall method is understandable, but the four edge maps and condensed branch logic make it denser to read.",
        3,
        "There is one useful helper, but most of the algorithm still sits inline inside main.",
        4,
        "The use of dictionaries, stack traversal, and list processing fits normal Python practice.",
        3,
        "Tracking four separate edge maps works, but it adds noticeable bookkeeping and cognitive overhead.",
    ],
    [
        "Day13_Solution_2024.py",
        4,
        "The parser and search are easy to trace, although the manual string slicing is a bit brittle and noisy.",
        3,
        "Parsing is separated into a helper, but the rest of the workflow remains mostly monolithic.",
        3,
        "The brute-force loops are valid Python, but the parsing style is more manual than idiomatic.",
        4,
        "The solution is conceptually simple, with only modest extra noise from the manual parser.",
    ],
    [
        "Day13b_Solution_2024.py",
        3,
        "The helper names are descriptive, but the number-theory edge cases and bound calculations make it fairly dense.",
        4,
        "The logic is broken into focused helpers, giving the script a much stronger structure than the more monolithic files.",
        4,
        "Integer arithmetic, small helpers, and explicit edge-case handling are used in a sound Python style.",
        3,
        "Much of the complexity is problem-driven, but the degenerate-case machinery still makes the solution fairly heavy.",
    ],
    [
        "Day24_Solution_2024.py",
        4,
        "The recursive evaluator is short and easy to follow, with sensible names and little clutter.",
        3,
        "There is a clear helper for evaluation, but parsing and result assembly still remain at module scope.",
        4,
        "Recursive memoization via dictionaries is a natural Python approach for this circuit evaluation.",
        4,
        "The memoized recursion stays focused on the problem without extra abstraction.",
    ],
    [
        "Day24b_Solution_2024.py",
        3,
        "The high-level idea is traceable, but the many special-case branches and intermediate wire names make it hard to scan quickly.",
        4,
        "Parsing, lookup, and orchestration are separated into sensible helpers and a clear main flow.",
        4,
        "Dictionary-based reverse lookup and string formatting are reasonable Python tools for this analysis.",
        2,
        "The branch-heavy swap detection feels heuristic and carries a lot of control-flow complexity.",
    ],
    [
        "Day25_Solution_2024.py",
        4,
        "The lock-key comparison is simple to read, though the nested loops and mutable flag are slightly old-fashioned.",
        2,
        "Everything is kept in top-level script code with no helper functions or separation of phases.",
        4,
        "The loops and list accumulation are ordinary, appropriate Python for a small puzzle script.",
        5,
        "The approach is minimal and direct, with no real overengineering.",
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

    output = Path("LLM-as-a-judge_2024.xlsx")
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
