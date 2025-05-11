# MERIL DIAGNOSTICS PVT LTD

# AutoQuant 100/100i/200i/400i ASTM LIS Communication Protocol

**Version 1.2**

## Revision History

| Date | Version | Description / Modifications | Author |
|------|---------|----------------------------|--------|
| 09.Nov.2013 | 1.0 | Draft created | Chandrakant |
| 11.Nov.2013 | 1.1 | Added TCP/IP, multiple data retrieval and serial port cable configuration | Chandrakant |
| 18.May.2017 | 1.2 | Protocol updated with new changes | Naveen.K |

## Table of Contents

- [MERIL DIAGNOSTICS PVT LTD](#meril-diagnostics-pvt-ltd)
- [AutoQuant 100/100i/200i/400i ASTM LIS Communication Protocol](#autoquant-100100i200i400i-astm-lis-communication-protocol)
  - [Revision History](#revision-history)
  - [Table of Contents](#table-of-contents)
  - [1. Overview](#1-overview)
    - [1.1 Purpose](#11-purpose)
    - [1.2 Scope](#12-scope)
    - [1.3 Conventions](#13-conventions)
    - [1.4 Primary Reference](#14-primary-reference)
  - [2. Communication Specifications](#2-communication-specifications)
    - [2.1 Physical Layer](#21-physical-layer)
      - [2.1.1 Interconnection Diagram-Hardware](#211-interconnection-diagram-hardware)
    - [2.2 Data Link Layer](#22-data-link-layer)
      - [2.2.1 Communication Status](#221-communication-status)
      - [2.2.2 Transmission Characters](#222-transmission-characters)
      - [2.2.3 Checksum Calculation](#223-checksum-calculation)
    - [2.3 Record Transmission Protocol](#23-record-transmission-protocol)
      - [2.3.1 Message Header Record](#231-message-header-record)
      - [2.3.2 Patient Information Record](#232-patient-information-record)
      - [2.3.3 Test Order Record](#233-test-order-record)
      - [2.3.4 Result Record](#234-result-record)
      - [2.3.5 Comment Record](#235-comment-record)
      - [2.3.6 Request Information Record](#236-request-information-record)
      - [2.3.7 Message Terminator Record](#237-message-terminator-record)
  - [3. Actual Data Send and Receive through AutoQuant](#3-actual-data-send-and-receive-through-autoquant)
    - [3.1 Receive](#31-receive)
    - [3.2 Data Upload (Send)](#32-data-upload-send)

## 1. Overview

### 1.1 Purpose

This Document details the specifications for communication of Clinical Chemistry analyzer with LIS Software using ASTM protocol. It explains the process for communication through instrument software by user.

### 1.2 Scope

Detailed information on operation of the system is beyond the scope of this document. The information offered here is strictly to aid programmers in grasping very basic operational features of the Analyzer–LIS communication.

### 1.3 Conventions

This document consists primarily of a series of examples that show the information needed to successfully interface to the system. The basic concept of data transfer in this interface is the exchange of data and control frames between the host system and the analyzer.

### 1.4 Primary Reference

- E1381 – 02: Low-Level Protocol to Transfer Messages between Clinical Laboratory Instruments and Computer Systems.
- E1394 – 97: Standard specifications for Transferring Information between Clinical Instruments and Computer Systems.

## 2. Communication Specifications

Communication specifications are based on a layer protocol. The layers are:
- Physical Layer
- Data Link Layer
- Presentation Layer

### 2.1 Physical Layer

Specifies the sending and receiving of signals between the Analyzer PC and LIS PC through physical and electrical connections.

#### 2.1.1 Interconnection Diagram-Hardware

```
Analyzer PC <--> ASTM Communication Packets <--> TCP/IP <--> AutoQuant Analyzer <--> LIS PC
```

### 2.2 Data Link Layer

Specifies the sending and receiving of data by link connections and for each frame between Analyzer PC and LIS PC.

#### 2.2.1 Communication Status

Transition is accomplished through the following three phases:

**Establishment Phase**

Establishes a communication line, and determines the direction of data transfer.
The sender sends an [ENQ] signal to the receiver to respond to the sender, the receiver performs the following action:

```
ANALYZER <ENQ> LIS-PC
```

Returns an [ACK] signal when communications are enabled.

```
ANALYZER <ACK> LIS-PC
```

Returns a [NAK] if receiver is not ready (BUSY).

```
ANALYZER <NACK> LIS-PC
```

**Transfer Phase**

The sender transmits messages to the receiver until all messages are transferred.

```
ANALYZER <STX> Frame 1 <ETB> C1 C2 <CR> <LF> LIS-PC
         <ACK>
         <STX> Frame 2 <ETB> C1 C2 <CR> <LF>
         <ACK>
```

**Termination Phase**

The sender notifies the receiver that all messages have been transferred.
After the termination phase, the status returns to neutral.

The sender sends the <EOT> to inform the receiver that the message transmission has been completed.

```
ANALYZER <EOT> LIS-PC
```

When the sender sends <EOT>, sender goes into neutral status.
When the receiver receives <EOT>, receiver processes data packet and then gets into neutral status.

#### 2.2.2 Transmission Characters

| Sr. # | CHAR | HEX VALUE | Description |
|-------|------|-----------|-------------|
| 1 | STX | 02 | Receiver will slice data from this character onwards for actual data evaluation. |
| 2 | ETX | 03 | Receiver will slice data up to this character from STX for actual data evaluation. |
| 3 | ACK | 06 | Positive acknowledgment. Character used to confirm correct and complete string sent by the counterpart |
| 4 | NACK | 15 | Negative acknowledgment. Character used to confirm that string received is incorrect or incomplete |
| 5 | ENQ (BOT) | 5 | Character used to initiate communication. |
| 6 | EOT | 17 | Character used to confirm that transmission from the transmitting end is over. |
| 7 | ^ | 5E (decimal 94) | Component Delimiter |
| 8 | \` | 60 | Repeat Delimiter |
| 9 | & | 38 | Escape Delimiter |
| 10 | P,O,R,Q,C | -- | Record identification bytes:<br>P - Patient Information Record<br>O - Test Order Record<br>R - Result Record<br>C - Comment Record<br>Q - Request Information Record |

#### 2.2.3 Checksum Calculation

The checksum is the modulus 8 of the sum of ASCII values of the frame characters starting with and including 'FN' till character before <ETX> (in case of single frame) or <ETB> (in case of multiple frames).

### 2.3 Record Transmission Protocol

Specifies the messages that are sent and received by the Analyzer PC and LIS PC.

ASTM data is sent or received in terms of packets.
- Packet starts with the Header (H) and ends with the Terminator (L).
- Packet without header and terminator is treated as invalid and will be ignored.

**ASTM Record Types:**

| Sr No. | Record Type | Convention |
|--------|-------------|------------|
| 1 | Message Header Record | H |
| 2 | Patient Information Record | P |
| 3 | Test Order Record | O |
| 4 | Result Record | R |
| 5 | Comment Record | C |
| 6 | Request Information Record | Q |
| 9 | Message Terminator Record | L |

Fields marked as * are mandatory.

#### 2.3.1 Message Header Record

| Field | Message Header Record | Size |
|-------|----------------------|------|
| 1* | Record Type ID | H | 1 |
| 2* | Delimiter Definition | \|\`^& | 4 |
| 12 | Processing ID | P: (Production) Treat message as an active message to be completed according to standard processing. | 1 |
| 13 | Version No. | ASTM version No. 1394-97 | 10 |
| 14 | Date and Time of message | current date time YYYYMMDDHHMMSS | 14 |
| 15* | Carriage Return | <CR> End of the string | 1 |

**Example String:**
```
H|\^&||||||||||P|E 1394-97|20100705071134<CR>
```

**About Delimiters:**
1. | - Field Delimiter (Alt + 124)
2. \` - Repeat Delimiter (Alt + 96)
3. ^ - Component Delimiter (Alt + 94)
4. & - Escape Delimiter (Alt + 38)

#### 2.3.2 Patient Information Record

| Field | Patient Information Record | Size |
|-------|---------------------------|------|
| 1* | Patient Record | P | 1 |
| 2* | Sequence Number | Frame Number (only 1 digit) | 1 |
| 3 | Practice Assigned Patient ID | Patient ID | 40 |
| 6 | Patient Name | Name of the Patient (Last Name^First Name^Middle Name^Title). If Patient Name contains single quote i.e." '" then it will get replaced by "\`" while saving data. | 30 |
| 8 | BirthDate | YYYYMMDDHHMMSS | 14 |
| 9 | Patient Sex | M/F/U (Male/Female/Other) | 1 |
| 11 | Patient Address | Street Address^City^State^Zip^Country Code. If Patient Address contains single quote i.e." '" then it will get replaced by "\`" while saving data. | 50 |
| 13 | Patient Telephone No. | Phone1\`Phone2\`Phone3 (It may contain area code, countrycode, beeper number, hours to call) e.g. +912212345678\`+912212345679 | 20 |
| 14 | Attending Physician ID | (Ordering Physician \` Attending Physician \` Referring Physician). If Physician ID contains single quote i.e." '" then it will get replaced by "\`" while saving data. | 40 |
| 17 | Height | Height/Weight and Unit are separated by component delimiter. 1.2^M (Default unit is cms for ht and Kg for wt). | 8 |
| 18 | Weight | | 7 |
| 36* | Carriage Return | <CR> | 1 |

**Example String:**
```
P|1|patient1|||VICHARE^PAT1^V||19710704|M||ANDHERI^MAHARASHTRA|RES1|8756873`694749387948|NENE^RAM|||1.2^M|23<CR>
```

#### 2.3.3 Test Order Record

| Field | Test Order Record | Size |
|-------|------------------|------|
| 1* | Test Order Identifier | O | 1 |
| 2* | Sequence Number | Frame No. | 1 |
| 3* | Specimen ID | Sample ID^Container No. (Samp1^01) Values of Container No: 1 = TUBE (10 ml) – Default Value, 3 = TUBE (5-7 ml) | 25 |
| 5* | Universal Test ID | Test Name (^^^ALB\`^^^ALP\`^^^LIVER) | 250 |
| 6* | Priority | S /A : Stat OR As soon as possible [i.e. Emergency], R : Routine | 1 |
| 8 | Specimen collection Date and Time | Actual date and time, the sample was collected (YYYYMMDDHHMMSS) | 14 |
| 11 | Collector ID | The person and facility which collected the specimen. If Collector ID contains single quote i.e." '" then it will get replaced by "\`" while saving data. | 20 |
| 12* | Action Code | A : Add the requested tests or batteries to the existing sample, N : New requests accompanying a new sample, P : Pending sample (Add but don't schedule), C : Cancel request for the battery or tests named (Delete Test) | 1 |
| 15 | Date/ Time Specimen Received | Date and Time recorded by laboratory | 14 |
| 16* | Specimen Descriptor | Sample Type : Blood, Urine, Serum, Plasma, CSF (Not Case-Sensitive) | 6 |
| 32* | Carriage Return | <CR> | 1 |

**Example String:**
```
O|1|020100030286||^^^GLU`^^^UREA|R||||||A||||SERUM<CR>
```

**Important:**
Specimen collection Date and Time:
If YYYYMMDD part is not Numeric then Analyzer Software will save the data received with Sample Collection Date same as System Date.

#### 2.3.4 Result Record

| Field | Result Record | Size |
|-------|--------------|------|
| 1* | Result Record Identifier | R | 1 |
| 2* | Sequence Number | Frame No. | 1 |
| 3* | Universal Test ID | Test Name (^^^ALB) | 8 |
| 4* | Data or Measurement Value | Result value | 10 |
| 5 | Units | ISO 2955 | 20 |
| 6 | Reference Ranges | Lower limit to Upper limit | 30 |
| 7* | Result Abnormal Flags | | 50 |
| 8 | Nature of Abnormality Testing | N: Generic Normal Range was applied to all patients | 1 |
| 9 | Result Status | C: Correction of previously transmitted results (Patient Report), F: Final Results | 1 |
| 13* | Date / Time Test Completed | Result Date in YYYYMMDDHHMMSS format | 14 |
| 15* | Carriage Return | <CR> | 1 |

**Example String:**
```
R|1|^^^ALP|200|IU/L|DEFAULT|A|N|F||||20100513113450<CR>
R|2|^^^AMY|93|U/L|DEFAULT|N|N|F||||20100513113535<CR>
```

#### 2.3.5 Comment Record

| Field | Comment Record | Size |
|-------|---------------|------|
| 1* | Comment Record Identifier | C | 1 |
| 2* | Sequence Number | Frame No. | 1 |
| 3* | Comment Source | L: Computer System (LIS), I: Instrument (ASTM) | 1 |
| 4* | Comment Text Code | ^Comment Text | 1000 |
| 5* | Comment Type | G: Generic/Free text comment, T: Test Name comment | 1 |
| 6* | Carriage Return | <CR> | 1 |

**Example String:**
```
C|1|I| Test ALB Does Not Exist For SampleID 01010125.|G<CR>
```

#### 2.3.6 Request Information Record

| Field | Request Information Record | Size |
|-------|---------------------------|------|
| 1* | Request Record Identifier | Q | 1 |
| 2* | Sequence Number | Frame No. | 1 |
| 3* | Starting Range ID Number | SampleID1\`SampleID2 | 115 |
| 6 | Nature of Request Time Limits | S: Sample Collection Date, R: Result Test Date | 14 |
| 13* | Request Information Status Codes | O: Requesting test orders and demographics only | 1 |
| 15* | Carriage Return | <CR> | 1 |

**Example String:**
```
Q|1|^020100030279`020100030321`020100030304`020100030297|||S|||||||O<CR>
```

#### 2.3.7 Message Terminator Record

| Field | Message Terminator Record | Size |
|-------|--------------------------|------|
| 1* | Message Terminator Record Identifier | L | 1 |
| 2* | Sequence Number | 1 | 1 |
| 3* | Termination Code | N: Normal Termination | 1 |

**Example String:**
```
L|1|N<CR>
```

**Data packets format example:**

**Patient Request (Host to LIS):**
```
<ENQ>(LIS to Host)
<ACK> (Host-> LIS)
<STX>1H|`^&||****|TBM-LIMS|Seepz||||||E-1394-97|20131205090513<CR>P|1|PAT1|LPAT1|LPAT13 |Joshi^Pramila^V||19710704|M|W|ANDHERI^MAHARASHTRA|RES1|8756873`69749387948|`NENE^RAM|SP1|SP2|1.2^M|23|PDIG1|PACTMED|DIET|PR1|PR2|20080929`20080929|OP|ANDHERI|NAltDig|AltDig|H|M|ARP|marathi|HpSer|HpInst|A<CR>C|1|L|Patient Information|G<CR>O|1|Pat1|IPat1|^^^ABCD1`^^^ALB`^^^TBIL|R|20080929|20080929|20080929|200^ml|preeta|N|DngC|RCIInfo|20080929|SERUM|`NENE^RAM|233245354|||LB1|LB2|20080929|566|B1|O|RES1|WARD1|NIF|SPSER|SPINST<CR>L|1|N<CR><ETX>6B<CR><LF> (LIS to Host)
<ACK> (Host-> LIS)
<EOT> (LIS to Host)
```

**Result Packet (Host to LIS):**
```
<ENQ><ACK>
<STX>1H|\^&|||Meril^3.6^11052213||||||||E-1394-97|20131203141051<CR><ETX>28<CR><LF><ACK>
<STX>2P|1||||Chan Du|||M||||||25^Y<CR><ETX>49<CR><LF><ACK>
<STX>3R|1|^^^TP|10.00|g/dL|0^0|||N|F||||20131203141051<CR><ETX>0F<CR><LF><ACK>
<STX>4R|2|^^^ALB|5.00|g/dL|0^0|||N|F||||20131203141051<CR><ETX>10<CR><LF><ACK>
<STX>5L|1|N<CR><ETX>06<CR><LF><ACK>
<EOT><ACK>
```

## 3. Actual Data Send and Receive through AutoQuant

### 3.1 Receive

For TCP/IP settings:
- Go to Maintenance > parameter settings > Enter password > enter TCP/IP details with port number > Select.

For serial port or TCP/IP communication:
- Go to Schedule screen, click the LIS button
- 'LIS information' window will open

For Serial port:
- Select the port number and Baud rate.
- Select appropriate com-port, on which serial port of LIS is connected.
- Baud Rate will be selectable during LIS communication. Same baud rate should be set at LIS Software.

For TCP/IP:
- Select TCP/IP and confirm the IP address and IP port. Start receiving directly.
- Click on Receive button.
- On click patient Input Dialog will open.

Workflow:
1. Enter patient ID for patient information to be received and click on save. Multiple patient ID can be saved similarly.
2. After save, Receive button will be enabled.
3. Click on Receive.
4. Application will send all patient ID data to LIS. In response, LIS will send patient information & test order. Application will store this data & display it on screen.
5. All details are shown in respective fields. Instrument sample ID and position will be automatically assigned by the software based on already scheduled or run samples for that day.
6. User can confirm all the details. If required to change Position and container type, user can select that patient, change position and type, click OK, and modified information will be reflected for that sample.
7. Data will be reflected in list. Click Save to schedule the details.
8. Patient information receive process is completed. Work list can be seen.
9. User can run the schedule as per normal procedure.

### 3.2 Data Upload (Send)

After Run completion, results are displayed on report screen:
1. Select Patient ID to send the results to LIS.
2. Add information (details) if required, click saves and click on Data upload button.
3. If required, multiple patient data can be selected by dragging mouse over required IDs.
4. It will ask for LIS communication details; select the fields.
5. LIS Port has to be selected.
6. Click on Select. Data will be uploaded to LIS.
7. Completion message will be displayed in screen.