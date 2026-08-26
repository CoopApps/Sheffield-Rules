# Genealogy CSV Search Results

## Objective
Find male players in Sheffield1867.db who have household member information stored, and check if those household members (especially women) appear in the genealogy CSV files.

## Key Findings

### 1. Database Structure
The `sheffield_players` table contains the following census-related fields:
- `census_household_schedule` - Household schedule number
- `census_relation` - Relation to head of household
- `census_gender` - Gender (Male/Female)
- `census_age` - Age at time of census
- `census_ed`, `census_piece`, `census_folio`, `census_page` - Census reference numbers
- `ecclesiastical_parish` - Church parish

### 2. Players with Household Data
Found **20 players** with `census_household_schedule` data populated:
- Most are born in 1837 or 1853
- All in St Philip parish
- Examples: Marshall Bragden, John Martin, Charles Hall, George Booth, etc.

### 3. Female Records in Database
Total female records: **20** (out of thousands of male players)

Female records with birth years in genealogy CSV range (1827-1838):
1. **Hannah Slatter** (b. 1828) - Relation: Head, Household: 185, Parish: Albert Memorial
2. **Elizabeth Brook** (b. 1831) - Relation: Head, Household: 26, Parish: Parish Church
3. **John George** (b. 1832) - Relation: Head, Gender: Female (data error)
4. **George Wrona** (b. 1838) - Relation: Mother-in-law, Household: 110, Parish: Christchurch

### 4. Data Quality Issues
The database has significant data quality problems:
- **No actual household structures**: Households contain multiple people all marked as "Head"
  - Example: Household 133 (St Philip) has 19 members, all marked as "Head"
  - This suggests census data was imported incorrectly or represents a different data structure
- **Gender inconsistencies**: Males marked as "Daughter" or "Wife" in some cases
  - Example: Samuel Jubb (male) marked as "Daughter"
- **Only 2 female records** properly marked as "Wife" or "Daughter" in entire database

### 5. Genealogy CSV Coverage
The genealogy CSV files cover birth years **1827-1838** only:
- Files are organized by birth year and surname initial
- Format: `tg_YEAR_INITIAL.csv`
- Example: `tg_1827_A (1).csv`, `tg_1831_E.csv`

### 6. Matching Attempts

#### Elizabeth Brook (b. 1831)
**Database record:**
- Name: Elizabeth Brook
- Birth: 1831
- Relation: Head
- Gender: Female
- Household: 26, Parish Church
- Birth Place: Sheffield, Yorkshire, England

**Genealogy CSV matches found:**
1. "Elizabeth Brookes" (1831) - Grenoside, Yorkshire - **Wife** - in tg_1831_E.csv
2. "Elizabeth Brook" (1830) - Brosley, Shropshire - **Wife** - in tg_1830_E.csv

**Analysis:** Possible match but different birth places and the database shows her as "Head" not "Wife"

#### Mary Slatter
**Genealogy CSV match:**
- "Mary Slatter" (1837) - 47 Peel Street, Sculcoates - **Wife** - in tg_1837_M.csv

**Note:** Database has "Hannah Slatter" (1828), not Mary. No direct match found.

## Conclusion

**The database does NOT contain actual household member data (wives, daughters, other family members) as separate records.**

What the database actually contains:
- Census reference information (`census_household_schedule`, etc.) for **male players only**
- These fields reference external census documents, not other database records
- The 20 female records appear to be either:
  - Data errors (males incorrectly marked as female)
  - A few standalone female individuals who happened to be included
  - Not actual family members of the male players

**To find household members:**
You would need to:
1. Take a male player's `census_household_schedule` number
2. Manually look up that household in the actual 1871 census documents
3. Find the wives/daughters/other family members listed there
4. Then search for those individuals in the genealogy CSVs

The genealogy CSVs appear to be extracted directly from census data and DO contain family member information (wives, daughters, etc.), but this information is not linked to the Sheffield1867.db player records.

## Example for Future Research

If you want to find a player's family members:
1. Player: **George Booth** (b. 1837)
2. Household Schedule: **133**
3. Parish: **St Philip** (probably from 1871 census)
4. Look up household 133 in 1871 Sheffield census for St Philip parish
5. Find his wife, children, etc. in that census household
6. Search genealogy CSVs for those family members by name and birth year

However, since the genealogy CSVs only cover 1827-1838, you won't find family members born outside this range (like children born after 1838 or wives born before 1827).
