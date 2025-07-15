# BF-6900 Automatic Hematology Analyzer
## Network Communication Interface Protocol V1.1

### A.1 Overview

This protocol is used for information transmission between BF-6900 Automatic Hematology Analyzer and LIS. It is based on HL7 standard, HL7 version is 2.4.

### A.2 Terms

**MSH**: Each MSH head part is used for defining message purpose and aim, each message is made up by several message segments. The first segment in each MSH is always the message head segment. It indicates the sending and receiving program name and message type, and only message ID code, and following segment structure is decided by message type. For example, a sample message send by OBR segment, one test result information send by many OBX segment.

**Segment**: Each message segment is made up by several group of date fields, each message segment has name, and it is used for bounding the content or function. Such as Message Header (MSH), patient information (PID), case history (PV1)

**Field**: Segment made by several date field. Different date field are separated by list separator.

### Syntax Format

```
<SB>dddd <EB><CR>
```

- **<SB>**: message start symbol (1byte). ASCII character<VT>, namely, 0x0B.
- **dddd**: data(made up by different length bytes). This is the HL7data content. Data could contain any byte value and ASCII code's carriage return symbol greater than hex value 0x1F ,<CR>.
- **<EB>**: message end character(1 byte). ASCII character <FS>, namely, `0x1C.
- **<CR>**: carriage return (1 byte). ASCII character<CR>, namely, 0x0D.

### Example:
```
<SB> MSH|^~\&|LIS|1234567890|||20100427194802||ORU^R01|1|P^S|2.4| <CR>
<EB><CR>
```

**Field Separators**: 5 characters after MSH are list separators used to differentiate each field, discreteness and sub-discreteness. Although those character could be any non-text character, but HL7 standard recommend following characters:

| Delimiter | Value |
|-----------|-------|
| Field Separator | \| |
| Discreteness Separator | ^ |
| Sub-Discreteness Separator | & |
| Repeat Separator | ~ |
| ESC | \ |

### A.3 Message Segments Used in this Protocol

- **MSH** – message head
- **PID** – patient information
- **PV1** – case history
- **OBR** – test report information
- **OBX** – test report test information
- **EQU** – instrument detail
- **NDS** - instrument affiche detail

### A.4 HL7 Attribute Table

Message segment in the protocol could be divided into required, optional, and repeatable.

## MSH Definition Table

MSH –message head: this message segment is required item，includes HL7 message basic information, message separator value, message type and message coding method and so on, it is each HL7 message's first message segment.

**Information Example:**
```
MSH|^~\&|BF-6900|1234567890|||20100419104618||ORU^R01|361|P^S|2.4|||||CHN|UNICODE<cr>
```

| Serial NO. | Field Name | Length | HL7 Advised Length | Explanation | Example |
|------------|------------|--------|-------------------|-------------|---------|
| 1 | Field Separator | 1 | 1 | Include the first field separator after message segment, used for regulating other message field separator value | \| |
| 2 | Coded Character | 4 | 4 | Include discreteness separator, repeat separator, ESC, sub-discreteness separator | ^~\& |
| 3 | Send Program | 7 | 180 | Send terminal apply program value：BF-6900 | BF-6900 |
| 4 | Instrument Code | 10 | 180 | Sending terminal instrument, value: instrument code | 1234567890 |
| 7 | Send Time | 14 | 26 | Message created time （form As YYYY[MM[DD[HH[MM[SS]]]]]），Take system time value | 20110310144704 |
| 9 | Message Type | 7 | 7 | Message type, form as "information type" event type, value: ORU^R01(Sample) OUL^R21 （LJ/X、XB QC） | ORU^R01 |
| 10 | Message Control ID | 20 | 20 | Message control ID is used for only mark one message, value :PID | 361 |
| 11 | Transact ID NO. | 3 | 3 | This field is used for decide on whether to transact HL7 operation program's(7th layer) transact rule definition information. Value: P^ message type（Type Value：S-sample、LJ-LJ /X barQC, XB-XB QC） | P^S |
| 12 | HL7 Version NO. | 3 | 60 | Agreements adopt HL7 version No.Value: 2.4 | 2.4 |
| 17 | Nation Code | 3 | 3 | Nation code mark, refer to HL 7 2.4 | CHN |
| 18 | Character Set | 10 | 10 | ISO/IEC 10646-1-1993 International character standard value: UTF-8 | UTF-8 |

## PID Definition Table

PID–patient information: this information segment is optional, used for patient sample transmission, include patient case history number, name, age, gender etc.

**Message Example:**
```
PID||1234567890||| Wang San Qiang||| M<cr>
```

| Serial NO. | Field Name | Length | HL7 Advice Length | Explanation | Example |
|------------|------------|--------|------------------|-------------|---------|
| 2 | Case History no. | 20 | 20 | Patient ID, here used for patient case history NO. | 1234567890 |
| 5 | Name | 50 | 250 | Patient name | Wang San Qiang |
| 8 | Gender | 10 | 1 | Gender, showed as character string | M |

## PV1 Definition Table

PV1 –patient in hospital information : This message segment is optional, use for patient sample transmission, include patient department, bed NO., deliver doctor, examiner and so on.

**Message example:**
```
PV1||| clinic^^235689|||| doctor Wang| Zhang San| Li Si<cr>
```

| Serial NO. | Field Name | Length | HL7 Advice Length | Explanation | Example |
|------------|------------|--------|------------------|-------------|---------|
| 3 | Pointed patient position | 80 | 80 | form as :department^^bed no. ^^clinic | 235689 |
| 7 | Deliver doctor | 50 | 250 | deliver doctor, character string | doctor Wang |
| 8 | Examiner | 50 | 250 | examiner, character string | Zhang San |
| 9 | Auditor | 50 | 250 | auditor, character string | Li Si |

## OBR Definition Table

OBR –testing report list information : This information segment is optional, mainly include test report information, include sample serial number, and scan No., tube rack No., deliver time and so on.

**Message example:**
```
OBR||23|31C3F010230DFB03|0001^Count Results||20071207080000|20071207160000|||||| |20071207083000||||2311|322<cr>
```

| Serial NO. | Field Name | Length | HL7 Advice Length | Explanation | Example |
|------------|------------|--------|------------------|-------------|---------|
| 2 | Sample Serial Number | 16 | 22 | Sample number in testing Document No. in LJ/X QC | 23 |
| 3 | Scan No. | 32 | 22 | Barcode ID in sample testing Lot No. in LJ/X QC | 31C3F010230DFB03 |
| 4 | Data Service Type | 200 | 200 | Service ID symbol, used for sign on different count result type. Idiographic value check the appendix OBR-4 message coding definition. | 0001^Count Results |
| 6 | Sample Time | 14 | 26 | Sampling time in testing. Validity in LJ/X quality control | |
| 7 | Count Time | 14 | 26 | Counting time in sample information Count time in LJ/X QC Count time in X-B quality control | |
| 14 | Delivery Time | 14 | 26 | delivery time. | |
| 18 | Tube Rack NO. | 2 | 60 | | |
| 19 | Tube NO. | 2 | 60 | | |

## OBX Definition Table

OBX –Test result: this message segment is repeatable item, mainly include all test result parameter information and sample test mode, analysis mode and reference group, etc.

**Message example:**
```
OBX|6|NM|2007^V_WBC||4.63|10*9/L|11.00-12.00|L|||F<cr>
```

| Serial NO. | Field Name | Length | HL7 Advice Length | Explanation | Example |
|------------|------------|--------|------------------|-------------|---------|
| 1 | Serial NO.ID | 10 | 10 | Used for mark different OBX message segment | 1 |
| 2 | Data Type | 3 | 3 | Test result's data type, value is "ST" 、 "NM" 、 "ED" 、 "IS" etc. | ED |
| 3 | ID Symbol | 250 | 250 | Test item mark. Form as "ID ^ Name", ID is test item mark, Name is test item descript information. Each test item serial no. value reference as appendix: identify coding definition. NOTE:ID used for only make one testing parameter, but name mainly for descript, not for mark. | |
| 5 | test result, chart data, notes, quality control level…… | 65536 | 65536 | Test result data, could be number, character string, enumerate value, binary system etc, data specific value reference the enumerate value table.( Binary data such as histogram and scatter plot, using Base64 encoding to do conversion) | |
| 6 | Unit | 10 | 250 | Unit, note: "^" in unit conflicts with discreteness separator, so use "*" to instead | 10*9/L |
| 7 | Test Result Reference Value | 20 | 60 | The scope of the test results, forms: "the reference range lower limit - upper limit of reference range" | 12.463-33.569 |
| 11 | Test Result Condition | 20 | 20 | Test result condition. Value is "F" - （Final Result）. Shows final test results | F |

## Custom Coding Definitions

This protocol uses the custom coding approach.

### OBR-4 Code Definition

| Code | Name | Explanation | OBR-4 Field |
|------|------|-------------|-------------|
| 1001 | Count Results | sample count result | 1001^ Count Results |
| 1002 | LJ QC | LJ QC count result | 1002^ LJ QC |
| 1004 | XB QC | XB QC count result | 1004^ XB QC |

### OBX-3 Identify Coding Definition

| Code | Name | Explanation | Value Type | OBX-3 Field |
|------|------|-------------|------------|-------------|
| 2001 | MODE | test mode | IS | 2001^MODE |
| 2002 | MODE_EX | analysis mode | IS | 2002^MODE_EX |
| 2003 | Ref | reference | IS | 2003^Ref |
| 2004 | Age | age | NM | 2004^Age |
| 2005 | Note | note | ST | 2005^Note |
| 2006 | Level | L-J/X QC level | IS | 2006^Level |
| 2007 | V_WBC | total white blood cell | NM | 2007^V_WBC |
| 2008 | V_BAS_c | The number of basophils | NM | 2008^V_BAS_c |
| 2009 | V_NEU_c | The number of neutrophils | NM | 2009^V_NEU_c |
| 2010 | V_EOS_c | The number of acidic granulocyte | NM | 2010^V_EOS_c |
| 2011 | V_LYM_c | The number of lymphocytes | NM | 2011^V_LYM_c |
| 2012 | V_MON_c | The number of mononuclear cells | NM | 2012^V_MON_c |
| 2013 | V_BAS_p | The percentage of basophils | NM | 2013^V_BAS_p |
| 2014 | V_NEU_p | The percentage of neutrophils | NM | 2014^V_NEU_p |
| 2015 | V_EOS_p | The percentage of eosinophils | NM | 2015^V_EOS_p |
| 2016 | V_LYM_p | Lymphocyte percentage | NM | 2016^V_LYM_p |
| 2017 | V_MON_p | percentage of Monocytes | NM | 2017^V_MON_p |
| 2018 | V_RBC | The number of red blood cells | NM | 2018^V_RBC |
| 2019 | V_HGB | Hemoglobin | NM | 2019^V_HGB |
| 2020 | V_MCV | MCV | NM | 2020^V_MCV |
| 2021 | V_MCH | Mean corpuscular hemoglobin | NM | 2021^V_MCH |
| 2022 | V_MCHC | Mean corpuscular hemoglobin concentration | NM | 2022^V_MCHC |
| 2023 | V_RDW_CV | Coefficient of variation of red blood cell distribution width | NM | 2023^V_RDW_CV |
| 2024 | V_RDW_SD | Standard deviation of red blood cell distribution width | NM | 2024^V_RDW_SD |
| 2025 | V_HCT | Hematocrit | NM | 2025^V_HCT |
| 2026 | V_PLT | Platelet count | NM | 2026^V_PLT |
| 2027 | V_MPV | Mean platelet volume | NM | 2027^V_MPV |
| 2028 | V_PDW | Platelet distribution width | NM | 2028^V_PDW |
| 2029 | V_PCT | Platelet hematocrit | NM | 2029^V_PCT |
| 2030 | V_P_LCR | Platelet - macrophage ratio | NM | 2030^V_P_LCR |
| 2101 | RBC Histogram.BIN | RBC scattergram BMP data | ED | 2101^RBC Scattergram.BMP |
| 2102 | PLT Histogram.BIN | PLT scattergram BMP data | ED | 2102^PLT Scattergram.BMP |
| 2103 | WBC Histogram.BIN | WBC scattergram BMP data | ED | 2103^WBC Scattergram.BMP |
| 2034 | DIFF Scattergram.BMP | DIFF scattergram BMP data | ED | 2034^DIFF Scattergram.BMP |
| 2104 | WBCD Scattergram.BMP | WBCD scattergram BMP data | ED | 2104^WBCD Scattergram.BMP |
| 2079 | XB_Num | How many quality control in XB to generate a quality control | NM | 2079^ XB_Num |

### Enumeration Type Values

| Data Item | Value |
|-----------|-------|
| test mode | 0- CBC, 1- CBC+DIFF |
| analysis mode | 0-open-whole blood, 1-open-pre-dilution, 2-auto-whole blood |
| reference | 0- normal, 1- M, 2- F, 3- Child, 4- baby, 5- custom 1, 6- custom 2, 7- custom 3, 8- custom 4, 9- custom 5 |
| L-J/X QC level | 0- high, 1- medium, 2- low |

## Complete Message Examples

### 1. Patient Sample

```
<SB> MSH|^~\&|BF-6900||||20110310150421||ORU^R01|8|P^S|2.4|||||CHN|UTF-8<cr>
PID||1234567890|||Wang Sanqiang|||Male<cr>
PV1|||门诊^^235689||||Doctor Wang|Zhang San|Li Si<cr>
OBR||2|12345|1001^ Count Results||20110310112251|20110310112409|||||| |20110310 112251||||0|0 <cr>
OBX|1|IS|2001^MODE||0||||||F<cr>
OBX|2|IS|2002^MODE_EX||1||||||F<cr>
OBX|3|IS|2003^Ref||0||||||F<cr>
OBX|4|IS|2004^Age||17|age|||||F<cr>
OBX|5|ST|2005^Note||note position||||||F<cr>
OBX|6|NM|2007^V_WBC||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|7|NM|2008^V_BAS_c||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|8|NM|2009^V_NEU_c||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|9|NM|2010^V_EOS_c||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|10|NM|2011^V_LYM_c||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|11|NM|2012^V_MON_c||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|12|NM|2013^V_BAS_p||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|13|NM|2014^V_NEU_p||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|14|NM|2015^V_EOS_p||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|15|NM|2016^V_LYM_p||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|16|NM|2017^V_MON_p||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|17|NM|2018^V_RBC||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|18|NM|2019^V_HGB||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|19|NM|2020^V_MCV||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|20|NM|2021^V_MCH||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|21|NM|2022^V_MCHC||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|22|NM|2023^V_RDW_CV||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|23|NM|2024^V_RDW_SD||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|24|NM|2025^V_HCT||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|25|NM|2026^V_PLT||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|26|NM|2027^V_MPV||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|27|NM|2028^V_PDW||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|28|NM|2029^V_PCT||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|29|NM|2030^V_P_LCR||4.63|10*9/L|11.00-12.00|L|||F<cr>
OBX|30|ED|2101^RBC Scattergram.BMP||……BMP binary system data change to BASE64 code……||||||F<cr>
OBX|31|ED|2102^PLT Scattergram.BMP||……BMP binary system data change to BASE64 code……||||||F<cr>
OBX|32|ED|2103^WBC Scattergram.BMP||……BMP binary system data change to BASE64 code……||||||F<cr>
OBX|33|ED|2034^DIFF Scattergram.BMP||……BMP binary system data change to BASE64 code……||||||F<cr>
OBX|34|ED|2104^WBC Scattergram.BMP||……BMP binary system data change to BASE64 code……||||||F<cr>
<EB><CR>
```

### 2. L-J/X QC

```
<SB>MSH|^~\&|BF-6900||||20110311091016||OUL^R21||P^LJ|2.4|||||CHN|TUF-8<cr>
OBR||2|123 |1002^ LJ QC||20100819 |20110217131356|||||| |||||0|0<cr>
OBX|1|IS|2006^Level||0||||||F<cr>
OBX|2|NM|2007^V_WBC||4.63||||||F<cr>
OBX|3|NM|2008^V_BAS_c||4.63||||||F<cr>
[... additional OBX segments ...]
<EB><CR>
```

### 3. X-B QC

```
<SB>MSH|^~\&| BF-6900||||20110311091040||OUL^R21||P^XB|2.4|||||CHN|UTF-8<cr>
OBR||||1004^ XB QC|||20071207160000||||||||||||<cr>
OBX|1|NM|2079^XB_Num||20||||||F<cr>
OBX|2|NM|2073^m_MCV_R||12.204||||||F<cr>
OBX|3|NM|2074^m_MCH_R||0.258||||||F<cr>
OBX|4|NM|2075^m_MCHC_R||12.445||||||F<cr>
OBX|5|NM|2076^m_MCV_L||45.859||||||F<cr>
OBX|6|NM|2077^m_MCH_L||1.258||||||F<cr>
OBX|7|NM|2078^m_MCHC_L||2.36||||||F<cr>
OBX|8|NM|2020^V_MCV||4.63||||||F<cr>
OBX|9|NM|2021^V_MCH||4.63||||||F<cr>
OBX|10|NM|2022^V_MCHC||4.63||||||F<cr>
<EB><CR>
```

---

*This document outlines the complete HL7 v2.4 implementation for the BF-6900 Automatic Hematology Analyzer's network communication interface protocol.*