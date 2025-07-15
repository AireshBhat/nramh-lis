# User Manual

## Appendix B Network Communication Interface Protocol V1.1

### A.1 Network Communication Interface Protocol V1.1

The protocol is applicable to information transmission between Automatic Hematology Analyzer (model: BF-6900) and the upper computer (LIS). It is in compliance with HL7 standard, and the HL7 version is 2.3.1.

### A.2 Definitions

**Message header (MSH)**: the first portion of each message is used to define the purpose and usage of messages, and each message consists of several sections. The first segment of a message is the message head segment which shows the program name, message type and the only message ID number for sending and reception, and the composition of the following segments is determined by the message type. For example, a sample message is transmitted by the section format OBR, and a test result message is transmitted by several OBX sections.

**Segment**: a message section consists of several data fields and each segment is provided with a name to define its contents and function. For example, MSH, PID, and PV1.

**Data field**: a message section consists of several data fields. The fields are separated by separators.

### A.3 Grammatical Format

```
<SB>dddd <EB><CR>
```

- `<SB>`: message start character (1 byte). ASCII character `<VT>`, i.e. 0x0B.
- `dddd`: data (composed of different length of bytes). This is HL7 data contents of the block. The data can contain any values of single byte greater than the hexadecimal value 0x1F and carriage return of ASCII code, `<CR>`.
- `<EB>`: message end character (1 byte). ASCII character `<FS>`, i.e. 0x1C.
- `<CR>`: carriage return (1 byte). ASCII character `<CR>`, i.e. 0x0D.

**Example:**
```
<SB> MSH|^~\&|BCC3900| X1706900BF0001|||20090419104618||ORU^R01|1|P|2.3.1|||||CHN|UTF-8 <CR>
<EB><CR>
```

**Where:**
The five characters after MSH are separators among data fields, assemblies and sub-assemblies. Although those characters are any non-text characters, the characters in the table below are recommended in HL7 standard:

| Delimiter | Value |
|-----------|-------|
| Data field separator | \| |
| Component separator | ^ |
| Sub-component separator | & |
| Repeating delimiter | ~ |
| Quoting character | \ |

### A.4 Message Section of the Protocol

1. **MSH** - Message header
2. **PID** - Patient information
3. **PV1** - Case
4. **OBR** - Testing report information
5. **OBX** - Testing report detection information
6. **MSA** - Response
7. **ORC** - Application

### A.5 HL7 Attribute List

The message sections of the agreement are classified as required, optional and repeatable.

#### A.5.1 MSH Definition List

**MSH** - message header: this section is required, including the basic information of HL7 message like the value of message separator, message type and encoding system, and it is the first section of each HL7 message.

**Message example:**
```
MSH|^~\&|BF-6900|0000011|||20180607102543||ORU^R01|1|P^S|2.4|||||CHS|UTF-8
```

| No. | Field name | Length | HL7 recommended length | Description | Examples |
|-----|------------|--------|------------------------|-------------|----------|
| 1 | Field separator | 1 | 1 | The first field separator after the name of the message section which defines the values of section separators of the other parts. | \| |
| 2 | Coded character | 4 | 4 | Component separator, repetition separator, escaping separator, and sub-component separator. | ^~\& |
| 3 | Sender program | 7 | 180 | Sender application program, value: XXX | XXX |
| 4 | Instrument code | 10 | 180 | Sender equipment, value: instrument code | 1234567890 |
| 5 | Receiving end program | 3 | 180 | Application program of receiving end. | LIS |
| 7 | Sending time | 14 | 26 | Time of current message. To call the time information of the system. Send time message to create time (in the form of YYYY[MM[DD[HH[MM[SS]]]]]), and take the system time value. For instance, 20110310144704 | 20090419104618 |
| 9 | Message type | 7 | 7 | Message type, like ORU^R01. The format is "Message type^Event type^Message structure name". Value: ORU^R01 (sample) | ORU^R01 |
| 10 | Message control ID | 20 | 20 | The message control ID is used to uniquely identify a message. Value: PID | 361 |
| 11 | Processing ID number | 3 | 3 | Process ID and always take P (indicating the query information of samples and work lists); Q (QC count results) | P |
| 12 | HL7 version No. | 3 | 60 | The protocol uses HL7 version No. Value: 2.3.1 | 2.3.1 |
| 18 | Character set | 10 | 10 | The worldwide character standard in ISO/IEC 10646-1-1993, value: UTF-8 | UTF-8 |

#### A.5.2 PID Definition List

**PID** - patient information: optional, used for transmission of patient sample, including case No., name, age and sex

**Message example:**
```
PID|1||7393670||Liu Jia|||F|||||||||||||||||||||||||25^Y<cr>
```

| No. | Field name | Length | HL7 recommended length | Description | Examples |
|-----|------------|--------|------------------------|-------------|----------|
| 1 | No. | 4 | 10 | Determine the message segment of different patients. | 1 |
| 3 | Case No. | 16 | 16 | Patient ID, the case No. here | 1234567890 |
| 5 | Name | 30 | 30 | Patient name | Wang Sanqiang |
| 8 | Sex | 1 | 1 | Sex Male, sent as M; sex Female, sent as F; and others sent as O. | F |
| 31 | Age | 5 | 5 | Age and age unit. The age and age unit are separated by ^. The age is a integer with length of 3. The age unit is a character string with length of 1. Y stands for year, M for month, D for day, and H for hour. | 25^Y |

#### A.5.3 PV1 Definition List

**PV1** - Patient admission information: This message segment is optional and is used for transmission of patient samples, including departments, charges, etc.

**Message example:**
```
PV1|1||Internal Medicine|||||||||||||||||||Self-paying<cr>
```

| No. | Field name | Length | HL7 recommended length | Description | Examples |
|-----|------------|--------|------------------------|-------------|----------|
| 1 | No. | 4 | 4 | It is used to mark the different PV1 sections of message. | 1 |
| 3 | Department | 80 | 80 | Patient location information, indicated in the form of "Department" | Outpatient |
| 20 | Expense type | 50 | 50 | Charge type, character string | Self paying |

#### A.5.4 OBR Definition List

**OBR** - Inspection report information: This message section is optional and mainly contains inspection report information, including sample numbers, inspection time, and so on.

**Message example:**
```
OBR||23|31C3F010230DFB03|1001^CountResults||20071207080000|20071207160000|||||| |20071207083000||||2311|322<cr>
```

| No. | Field name | Length | HL7 recommended length | Description | Examples |
|-----|------------|--------|------------------------|-------------|----------|
| 1 | No. | 4 | 10 | Determine different OBR fields. | 1 |
| 2 | Bar code | 22 | 22 | Number of the doctor's advice for the requester, used as the sample bar code No. Bar code ID for detection Lot number in L-J/Xbar/Xbar-R QC | 1A2165C24B |
| 3 | Sample No. | 12 | 12 | Number of doctor's advice of the executor, used as the sample No.Sample ID for detection File No. in L-J/Xbar/Xbar-R QC | 20090807011 |
| 4 | Date servicing type | 200 | 200 | Service mark, for identifying different counting results. Specific values and parameter are shown in the appendix: OBR-4 message code definition. | 1001^CountResults |
| 6 | Sampling time | 14 | 26 | Request time/date. Sampling time in sample detection Validity in LJ/Xbar/Xbar-R QC | 20090807140600 |
| 7 | Counting time | 14 | 26 | Counting time in sample information Counting time in LJ/Xbar/Xbar-R QC Counting time in X-B QC | 20090807150616 |
| 10 | Submitter | 8 | 8 | Submitter | Dr |
| 14 | Date of submitting | 14 | 26 | Date of submitting | 20090807150000 |
| 20 | Inspecting doctor | 30 | 30 | Attending doctor, for inspecting doctor | Dr |
| 28 | Auditor | 150 | 150 | Copy of results for reviewers | Dr |

#### A.5.5 OBX Definition List

**OBX** - Test result: repeatable, mainly including the test result parameters, analysis modes, reference groups, etc.

**Message example:**
```
OBX|6|NM|2007^V_WBC||4.63|10*9/L|11.00-12.00| |||F|| <cr>
```

| No. | Field name | Length | HL7 recommended length | Description | Examples |
|-----|------------|--------|------------------------|-------------|----------|
| 1 | Serial No. ID | 10 | 10 | It is used to mark the different OBX sections of message. | 1 |
| 2 | Data type | 3 | 3 | Date type of test result, including ST, NM, ED and IS | ED |
| 3 | Identifier | 250 | 250 | Test item mark. The form is ID^Name, ID is the test item mark, and the Name is description of test item. The coding value of testing items is shown in appendix: definition of identifier code. Note: ID is the only way to confirm the test parameter, but the Name is used for description instead of marking. | |
| 5 | Test result, graphic data, remarks, QC level... | 65536 | 65536 | Test result data can be numbers or words. Strings, enumeration values, binary data, and so on [Histograms and scatterplots and the like converted using Base64 encoding] Remarks: Xbar-R QC data format is mean^limit. | 4.63 Xbar-r 5.5^1.1 |
| 6 | Unit | 12 | 250 | Unit, used for the test result value. ISO standard unit is adopted. | 10*9/L |
| 7 | Reference value of test results | 30 | 60 | Test result range, format: "lower limit-upper limit of reference value | 12.463-33.569 |
| 11 | Test result status | 20 | 20 | Test result status. The value is F - (Final Result) which means the final result. | F |

#### A.5.6 ORC Definition Table

**ORC** - Message segment mainly contains Order-related information.

**Message example:**
```
ORC|RF||SampleID||IP <cr>
```

| No. | Field name | Length | HL7 recommended length | Description | Examples |
|-----|------------|--------|------------------------|-------------|----------|
| 1 | Order control word | 2 | 2 | Value: RF in ORM indicates request; AF in ORR message indicates confirmation. | RF |
| 2 | Originator number of Order | 22 | 22 | Originator number of Order ORM message is empty, while ORR message is bar code number. | |
| 3 | Receiver number of Order | 22 | 22 | Receiver number of Order There is a bar code number in the ORM message and empty value in the ORR message. | SampleID |
| 5 | Order status | 2 | 2 | Through worklist query, the value in ORM message is IP, indicating Order is in processing and no results have been got yet. No value in ORR. | IP |

#### A.5.7 MSA Definition Table

The MSH segment of the interface includes the following domains:

| No. | Field name | Length | HL7 recommended length | Description | Examples |
|-----|------------|--------|------------------------|-------------|----------|
| 1 | Confirmation code | 2 | 2 | Confirmation code, AA indicating acceptance, AE error, and AR rejection | AA |
| 2 | Message control ID | 20 | 20 | The message control ID is same to the MSH-10 of the sender. | 1 |
| 6 | Error condition | 100 | 100 | Error condition (status code).See the figure below for details | |

**The value of MSA-6 field is shown in the table below:**

| Status code (MSA-6) | Status text (MSA-3) | Description/Remark |
|---------------------|----------------------|-------------------|
| **Success** | **AA** | |
| 0 | Message accepted | Succeeded |
| **Error status code** | **AE** | |
| 100 | Segment sequence error | Sequence of middle message segment is incorrect or necessary fields are missing. |
| 101 | Required field missing | Required fields in a segment are missing. |
| 102 | Data type error | Field data type error, like the figure is written in character. |
| 103 | Table value not found | Tabular value not found, not used temporarily. |
| **Status code rejected** | **AR** | |
| 200 | Unsupported message type | Message type not supported. |
| 201 | Unsupported event code | Event code not supported. |
| 202 | Unsupported processing id | Handling ID not supported. |
| 203 | Unsupported version id | Version ID not supported. |
| 204 | Unknown key identifier | Unclear key identifier, like transmitting the information of a nonexistent patient. |
| 205 | Duplicate key identifier | Existing repeated key identifier. |
| 206 | Application record locked | Affair cannot be executed in application program storage level, like database being locked. |
| 207 | Application internal error | Other unknown internal error of application. |

The protocol applies the custom coding mode.

### A.6 OBR-4 Coding Definition

| Code | Name | Description | OBR-4 field |
|------|------|-------------|-------------|
| 1001 | Count Results | Sample count results | 1001^CountResults |
| 1002 | LJ QC | L-J QC count results | 1002^LJQC |
| 1003 | Xbar QC | Xbar QC count results | 1003^XbarQC |
| 1004 | XB QC | X-B QC count results | 1004^XBQC |
| 1005 | CRP QC | CRP QC count results | 1005^CRPQC |
| 1006 | XbarR QC | Xbar-R QC count results | 1006^XbarRQC |

### A.7 OBX-3 Identifier Code Definition

| Code | Name | Description | Value type | OBX-3 field |
|------|------|-------------|------------|-------------|
| 2001 | MODE | Analysis mode | IS | 2001^MODE |
| 2002 | MODE_EX | Measurement mode | IS | 2002^MODE_EX |
| 2003 | Ref | Reference group | IS | 2003^Ref |
| 2004 | Note | Remarks | ST | 2004^Note |
| 2005 | Level | L-J/Xbar/Xbar-R/CRP QC level | IS | 2005^Level |
| 2006 | V_WBC | Total number of white blood cell | NM | 2006^V_WBC |
| 2007 | V_NEU_p | Percentage of neutrophil | NM | 2007^V_NEU_p |
| 2008 | V_LYM_p | Percentage of lymphocyte | NM | 2008^V_LYM_p |
| 2009 | V_MON_p | Percentage of monocyte | NM | 2009^V_MON_p |
| 2010 | V_EOS_p | Percentage of eosinophil | NM | 2010^V_EOS_p |
| 2011 | V_BAS_p | Percentage of basophil | NM | 2011^V_BAS_p |
| 2012 | V_NEU_c | Number of neutrophil | NM | 2012^V_NEU_c |
| 2013 | V_LYM_c | Number of lymphocyte | NM | 2013^V_LYM_c |
| 2014 | V_MON_c | Number of monocyte | NM | 2014^V_MON_c |
| 2015 | V_EOS_c | Number of eosinophil | NM | 2015^V_EOS_c |
| 2016 | V_BAS_c | Number of basophil | NM | 2016^V_BAS_c |
| 2017 | V_RBC | Number of red blood cell | NM | 2017^V_RBC |
| 2018 | V_HGB | Hemoglobin | NM | 2018^V_HGB |
| 2019 | V_MCV | Mean red blood cell volume | NM | 2019^V_MCV |
| 2020 | V_HCT | RBC hematocrit | NM | 2020^V_HCT |
| 2021 | V_MCH | Mean red blood cell hemoglobin content | NM | 2021^V_MCH |
| 2022 | V_MCHC | Mean red blood cell hemoglobin concentration | NM | 2022^V_MCHC |
| 2023 | V_RDW_SD | Standard deviation of red blood cell distribution width | NM | 2023^V_RDW_SD |
| 2024 | V_RDW_CV | Red blood cell distribution width variation coefficient | NM | 2024^V_RDW_CV |
| 2025 | V_PLT | Number of platelet | NM | 2025^V_PLT |
| 2026 | V_MPV | Average platelet volume | NM | 2026^V_MPV |
| 2027 | V_PCT | Platelet hematocrit | NM | 2027^V_PCT |
| 2028 | V_PDW | Platelet distribution width | NM | 2028^V_PDW |
| 2029 | V_P_LCR | Platelet - ratio of macrophage | NM | 2029^V_P_LCR |
| 2030 | V_P_LCC | Platelet ratio | NM | 2030^V_P_LCC |
| 2031 | V_CRP | C reactive protein | NM | 2031^V_CRP |
| 2032 | V_HS_CRP | Hypersensitive C-reactive protein | ST | 2032^V_HS_CRP |
| 2101 | RBCHistogram.PNG | RBC histogram PNG data | ED | 2101^RBCScattergram.PNG |
| 2102 | PLTHistogram.PNG | PLT histogram PNG data | ED | 2102^PLTScattergram.PNG |
| 2033 | BASOScattergram.PNG | BASO scattergram PNG data | ED | 2033^BASOScattergram.PNG |
| 2034 | DIFFScattergram.PNG | DIFF scattergram PNG data | ED | 2034^DIFFScattergram.PNG |

### A.8 Enumeration Type List

| Data item | Value |
|-----------|-------|
| Analysis mode | 0 - Whole blood 1 -Trace whole blood 2 -Pre-dilution |
| Measurement mode | 0 -CBC 1-CBC+DIFF 2-CBC+DIFF+CRP 3- CRP |
| Reference group | 0 - Normal 1 - Male 2 - Female 3 - Child 4 - Newborn 5 -Self-defined 16- Self-defined 27 -Self-defined 38 -Self-defined 4 9 -Self-defined 5 |
| QC level | 0-high 1-middle 2-low |

## Example of a Complete Message Section:

### (1) Patient Sample

```
<SB>MSH|^~&|BF-6900|20180613001|LIS||20110613153322||ORU^R01|3|P|2.3.1||||||UTF-8<cr>
PID|1|||||||U|||||||||||||||||||||||1^Y<cr>
PV1|1|||||||||||||||||||<cr>
OBR|1||5|1001^CountResults||20180601091634|20180601091637|||||||20180601091634||||||---||||||||<cr>
OBX|1|IS|2001^MODE||0||||||F||<cr>
OBX|2|IS|2002^MODE_EX||1||||||F||<cr>
OBX|3|IS|2003^Ref||0||||||F||<cr>
OBX|4|IS|2004^Note||||||||F||<cr>
OBX|5|NM|2006^V_WBC||0|10^9/L|4-10||||F||<cr>
OBX|6|NM|2007^V_NEU_p||0|%|50-70||||F||<cr>
OBX|7|NM|2008^V_LYM_p||0|%|20-40||||F||<cr>
OBX|8|NM|2009^V_MON_p||0|%|3-8||||F||<cr>
OBX|9|NM|2010^V_EOS_p||0|%|0.5-5||||F||<cr>
OBX|10|NM|2011^V_BAS_p||0|%|0-1||||F||<cr>
OBX|11|NM|2012^V_NEU_c||0|10^9/L|2-7||||F||<cr>
OBX|12|NM|2013^V_LYM_c||0|10^9/L|0.8-4||||F||<cr>
OBX|13|NM|2014^V_MON_c||0|10^9/L|0.12-0.8||||F||<cr>
OBX|14|NM|2015^V_EOS_c||0|10^9/L|0.02-0.5||||F||<cr>
OBX|15|NM|2016^V_BAS_c||0|10^9/L|0-0.1||||F||<cr>
OBX|16|NM|2017^V_RBC||0|10^12/L|3.5-5.5||||F||<cr>
OBX|17|NM|2018^V_HGB||1|g/L|110-160||||F||<cr>
OBX|18|NM|2019^V_MCV||0|fL|80-100||||F||<cr>
OBX|19|NM|2020^V_HCT||0|L/L|0.35-0.5||||F||<cr>
OBX|20|NM|2021^V_MCH||0|pg|27-34||||F||<cr>
OBX|21|NM|2022^V_MCHC||0|g/L|320-360||||F||<cr>
OBX|22|NM|2023^V_RDW_SD||0|fL|35-56||||F||<cr>
OBX|23|NM|2024^V_RDW_CV||0|%|11-16||||F||<cr>
OBX|24|NM|2025^V_PLT||0|10^9/L|100-300||||F||<cr>
OBX|25|NM|2026^V_MPV||0|fL|7-13||||F||<cr>
OBX|26|NM|2027^V_PCT||0|%|0.1-0.28||||F||<cr>
OBX|27|NM|2028^V_PDW||0|fL|15-18||||F||<cr>
OBX|28|NM|2029^V_P_LCR||0|%|13-43||||F||<cr>
OBX|29|NM|2030^V_P_LCC||0|10^9/L|13-129||||F||<cr>
OBX|30|NM|2031^V_CRP||0|mg/L|0-6||||F||<cr>
OBX|31|ST|2032^V_HS_CRP||0.00|mg/L|0-6||||F||<cr>
OBX|32|ED|2101^V_RBCScattergram.PNG||PNG binary data converted into BASE64 coding||||||F||<cr>
OBX|33|ED|2102^V_PLTScattergram.PNG||PNG binary data converted into BASE64 coding||||||F||<cr>
OBX|34|ED|2033^V_BASOScattergram.PNG||PNG binary data converted into BASE64 coding||||||F||<cr>
OBX|35|ED|2034^V_DIFFScattergram.PNG||PNG binary data converted into BASE64 coding||||||F||<cr>
<EB><CR>
```

### (2) L-J QC

```
<SB>MSH|^~&|BF-6900|20180613001|LIS||20110613153445||ORU^R01|5|Q|2.3.1||||||UTF-8<cr>
PID|1||||||||||||||||||||||||||||||^<cr>
PV1|1|||||||||||||||||||<cr>
OBR|1|||1||1002^LJQC|20180420|||20180420113307||||||||||||||||||<cr>
OBX|1|IS|2005^Level||1||||||F||<cr>
OBX|2|NM|2006^V_WBC||465.11|10^9/L|490-510||||F||<cr>
[... additional OBX lines with test values ...]
<EB><CR>
```

### (3) Worklist Application

```
<SB>MSH|^~&|BF-6900|20180613001|LIS||20110613153408||ORM^O01|4|P|2.3.1||||||UTF-8<cr>
ORC|RF||218||IP<cr>
<EB><CR>
```

### (4) Worklist Obtaining

```
<SB>MSH|^~\&|LIS||||20180613154025||ORR^O02|4|P^S|2.3.1||||||UTF8<cr>
MSA|AA|1||||0<cr>
PID|1||5||T5|||M|||||||||||||||||||||||3^Y<cr>
PV1|1||orthopedics|||||||||||||||||medical insurance<cr>
ORC|AF|218|||<cr>
OBR|1|218|5|1001^Count ||20180613153909||||Gu Yisheng||||20180613153919||||||||||||||<cr>
OBX|1|IS|2001^MODE||0||||||||<cr>
OBX|2|IS|2002^MODE_EX||0||||||||<cr>
OBX|3|IS|2003^Ref||0||||||||<cr>
OBX|4|ST|2004^Note||test||||||||<cr>
<EB><CR>
```