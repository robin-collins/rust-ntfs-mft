# How to Read and Parse the NTFS MFT

## Introduction

The NTFS file system is widely used in Windows operating systems such as Windows 10 and Windows Server. At the core of NTFS is the Master File Table (MFT). The MFT acts as an index for every file and directory stored on an NTFS volume. It contains file records that describe critical metadata about each file, including time stamps, security permissions, and physical storage locations.

Carefully analyzing and decoding these MFT file records can provide deep insight into an NTFS volume. However, directly reading the raw MFT requires in-depth technical knowledge to avoid data loss or corruption. This book provides readers with the necessary background, tools, and techniques to safely access, parse, and extract information from MFT file records.  

In particular, readers will learn:

- Key concepts of the NTFS file system architecture including attributes, data runs, and other metadata structures that are catalogued in the MFT

- The detailed low-level structure and format of MFT file records

- Considerations and best practices for directly reading the MFT to avoid data damage or loss

- How to parse and interpret common NTFS file record attributes including timestamps, security identifiers, and file data run information

- Techniques for handling errors, inconsistencies, resident vs non-resident attributes, and various data types when decoding MFT file records  

- Step-by-step walkthroughs of parsing sample MFT file records to identify and extract important file metadata

Whether you need to examine NTFS volumes forensically, troubleshoot file system issues, or simply want to learn more about NTFS and Windows internals, this book will help you gain the necessary knowledge and skills related to safely reading and navigating the intricacies of the MFT.

## 1.1 Brief History of NTFS File System  

The New Technology File System (NTFS) was introduced by Microsoft in 1993 as part of the Windows NT operating system. NTFS was designed to address limitations of the previous FAT file systems, and to provide improved reliability, security, and performance for server environments.

A key innovation introduced in NTFS is the Master File Table (MFT). The role of the MFT is to serve as a central index and metadata repository containing information about every file and directory stored on an NTFS volume. The MFT allows quick locating of files without having to traverse chains or trees. It also enables advanced capabilities like disk quotas, access control, and logging.

Internally, the MFT consists of file records which map uniquely to each file and store attributes describing that file. By analyzing these MFT file records, detailed information can be discovered about file contents, locations, timestamps, ownership and additional metadata.

NTFS has become the default file system used by recent Windows operating systems. However, the internal format used by NTFS for structures like the MFT are proprietary and not openly documented. Therefore reading and parsing the raw contents of entities like file records requires reverse engineering knowledge gained from experimentation and analysis.

As NTFS continues to evolve, the format has changed in some ways across versions, but maintains backward compatibility. Windows 10 currently uses NTFS version 3.1 as its file system, first introduced officially in Windows XP. By understanding specifications of the current NTFS format, files and file metadata can be decoded programmatically despite lack of public documentation.

While visibility into NTFS internals enables advanced forensics and data recovery use cases, caution must also be exercised. Direct access to structures like the MFT risks file system corruption if improperly handled. This guide will cover proper techniques for reading NTFS volumes at the raw level safely.

Here is a draft section for the introduction chapter overviewing key NTFS concepts and structures:

## 1.2 Overview of Key NTFS Concepts and Structures

NTFS employs several key concepts and structures to organize data on a volume. Understanding these building blocks is crucial for making sense of how information is stored and referenced in areas like the Master File Table.

Clusters
The cluster is the fundamental unit of allocation on an NTFS volume. A cluster represents a fixed number of sectors that serve as the smallest allocatable unit of disk space. The size of a cluster is set when a volume is formatted and ranges from 512 bytes up to 64KB, with 4KB being most common. Everything stored on an NTFS volume is allocated in cluster-sized blocks.

Metadata
Metadata refers to the filesystem structures that are used internally by NTFS to organize and manage information on a volume. This includes critical components like the Master File Table, which serves as an index for locating files. Other metadata structures allocate space, track bad sectors, and ensure consistency. Most metadata is kept in the first part of the volume.

Inodes
The inode is NTFS's internal representation of a file or directory. Each inode corresponds to a File Record stored in the Master File Table. The File Record contains attributes, timestamps, and data run information associated with that file or folder. Inodes allow tracking files as metadata instead of directly through sectors or clusters.
  
Master File Table (MFT)
The MFT is the primary metadata structure of NTFS volumes. It serves as an index and allocation table containing a File Record for every file and directory stored on the volume. The MFT uses File Records to describe the organization and content of each file. Information like file attributes, timestamps, sizes, and location on disk is stored in records within the MFT.

A backup of the first part of the MFT is also kept in a metadata file called $MFTMirr. This supports recovery if the primary MFT is corrupted or damaged. The MFT plays a central role in NTFS's ability to locate files and folders on a volume.

With an understanding of these core NTFS concepts, we can now explore more specifically how NTFS organizes content on a volume and how we can leverage critical structures like the MFT to access this information for parsing and analysis. The next section will provide an overview of common tools and techniques used for reading NTFS volumes at the metadata level.

## 1.3 Role and importance of the MFT

The Master File Table (MFT) is the most critical metadata structure in NTFS and essentially contains the entire definition of the file system. Each file and directory on an NTFS logical drive has a corresponding file record stored in the MFT.

Without the MFT, the NTFS file system would be an inaccessible collection of unidentified data clusters on disk. It is the MFT which provides the vital linkage between file contents, directories, and associated metadata needed for a usable file system.

More specifically, the MFT stores MFT records which contain attribute entries for every directory entry or raw data content area (either file or unnamed data stream) on the NTFS logical drive.  Some of key pieces of metadata stored for each file record include:

    File attributes: Standard attributes like name, timestamps, allocated size, data content runs also contain Extended Attributes and POSIX access control lists.
    Directory structure: Links between parent directory records allow traversing the on-disk directory tree.
    Security identifiers MFT records contain security IDs to identify ownership and access control roles on the file system.
    Application metadata: For example records to identifying bad sectors, transaction journals and replication information are stored.

Without all this aggregated, interlinked metadata the content of an MFT would not qualify as a usable file system. By centralizing critical file system metadata into the MFT, NTFS eliminates overhead traditionally needed to cross-reference standalone structures spread across disk. Everything needed to fully utilize and manage file system contents is stored in one place.

However all this centralization also introduces risks. Damage or corruption to the MFT risks the integrity of entire NTFS volumes. As such special precautions are utilized for updating the MFT and ancillary structures like the MFT mirror provide redundancy to allow repairs if issues are detected.

Here is a draft section on the risks of directly reading the MFT for a textbook introduction chapter:

## 1.4 Risks of Directly Reading the MFT

While the Master File Table contains a wealth of valuable metadata and content, directly reading the raw MFT comes with significant risks that must be properly understood and mitigated. Some of the key risks include:

File Corruption

The MFT contains the authoritative metadata that allows NTFS to access files and directories on the volume. If the raw MFT data is improperly read or written, subtle corruption could be introduced into file records. This can cause anything from invalid attributes to complete data loss if files and directories fail to load properly after modifications to metadata records. Care must be taken to never write or modify data when accessing the raw MFT directly. Read-only access is extremely important to avoid corruption.

System Instability

Because NTFS relies so heavily on the MFT being perfectly intact and accessible, introducing any instabilities when accessing the raw structure can lead to potential crashes, failed boots, or other serious system problems. The Windows cache manager in particular poses some complications, as it aggressively caches parts of the MFT and assumes exclusive ownership of the underlying file handles. Special considerations are necessary.

Disrupting Core Services  

Several components within Windows itself, as well as other third-party services rely on the MFT or signatures within file records. Parsing or modifying MFT contents incorrectly could disrupt essential Windows functions, anti-malware tools, backup software, and other processes that integrate tightly with the NTFS driver at a low level. This must be accounted for.

Data Loss

Since the metadata within the MFT enables access to user files and directories, improperly handling file records could make it impossible for Windows to locate files on the drive. This can lead to serious and irrecoverable data loss if backups are not available. While protections are in place for reliability, direct MFT access introduces new risks.

The techniques we will cover in this guide are designed to promote read-only raw access that avoids corruption or instabilities through controlled parsing. By understanding the above risks, we can put in necessary safeguards as we leverage the rich metadata within the MFT.

Here is a draft section on the tools that will be used in the book for reading and parsing the NTFS MFT:

## 1.5 Tools We Will Use in This Text

In order to read and parse the raw NTFS volume, we need tools that can provide low-level access for analysis while also helping ensure we do not corrupt the file system. We will rely on a few key tools in this text:

1. Libntfs - This open source library for working with NTFS volumes at the sector level allows us to access the volume read-only to protect the integrity of the data. The library parses aspects of the NTFS structures and metadata to make it easier for us to work with when analyzing the MFT.

2. Ntfsinfo - A utility that leverages libntfs to pull information directly from an NTFS volume and display details on attributes and metadata files like the MFT. We can use this to conveniently extract properties and review content during our parsing exercises.

3. Foremost - When digging deeper into attribute content and data streams, this utility helps carve out files based on headers and footers. While it won't help us parse the MFT itself, it will come in handy when trying to make sense of file content stored inside NTFS attributes we decode from the MFT.

4. Data Editor - In order to view the hex output from these tools and also experiment with manual parsing, we need a good hex editor. We will use a simple free utility to open raw data files and manually review and decode MFT records pulled from our NTFS volumes.

By leveraging these tools to read NTFS volumes in a read-only manner and then parse and display information from the MFT, we can walk through safe and reliable techniques for understanding the internals of NTFS file records. These utilities help us avoid corruption while also making the parsing process easier to explore.

## 2.1 Low-level Structure of the Master File Table

The Master File Table (MFT) is the central metadata repository in NTFS. Understanding its organization and contents is crucial for effectively reading and analyzing NTFS volumes.

Layout of MFT File Records

The MFT consists of file records that describe files and directories on the disk. The first 16 file records (0-15) are reserved for special metadata files used internally by NTFS. Additional file records then represent user files and directories.

Each 1KB file record has a standard 100-byte header containing flags, timestamps, and structural information. The header is followed by attributes that describe the file. The different attributes contain metadata (like security descriptors and filenames) along with information on the actual file data contents.

Header Details

Important values encoded in the standard file record header include:

    File record flags indicating if the record is in use
    A sequence number for tracking reuse of records
    The update sequence values used for error detection
    A hard link count quantifying the number of directory entries pointing to this file
    File record sizes detailing the logical vs allocated sizes
    Base file record information for records needing extension

Resident vs Non-Resident Attributes

One critical piece of information stored in each attribute header is whether the attribute is resident or non-resident.

Resident attributes can fit their actual value inside the MFT record itself. Non-resident attributes are too large, so the MFT record stores metadata about the attribute with the actual data contents stored in clusters on the disk.

Understanding this distinction is key when parsing an attribute. Resident attributes can be directly accessed from the file record, while non-resident attributes require processing its data run information to find the actual stored clusters. Getting this right is essential for locating file contents during analysis.

## 2.2 Common Metadata Files Stored in the MFT

The Master File Table (MFT) serves as an index containing a file record for every file and directory on an NTFS volume. In addition to user files and folders, the MFT also stores records for metadata files that are essential for the file system operations. Some of the key metadata files tracked in the MFT include:

$MFT - This record describes the MFT itself. It allows the MFT to be self-referencing.

$MFTMirr - Contains a backup copy of the first four file records in the MFT. This provides redundancy to help recover the volume if the MFT is corrupted.

$LogFile - Stores details of file system transactions for roll-back purposes. Used to restore volume consistency following a crash or improper shutdown.

$Volume - Holds information like the volume name, version, and "dirty" flags indicating file system integrity issues.

$AttrDef - Defines the set of attributes allowed for organizing file metadata on the volume. All other MFT records reference this global attribute list.

$Bitmap - Keeps track of cluster allocation across the volume by representing each one with a bit. Allows determining space usage and availability.

In addition to these core system files, the MFT will include records for the root directory, default system folders, and any user-created files and directories. But these common metadata files form the backbone enabling key functions like recoverability, transaction handling, and free space tracking. Their MFT records must be parsed to unlock many details of NTFS volumes.

## 2.3 Key Attribute Types

Attributes are used extensively within the NTFS file system to store metadata and information about files and directories. Within each file record in the Master File Table (MFT), attributes are used to encapsulate and organize the various metadata values and content associated with that file or directory.

Several attribute types appear commonly across most MFT file records and are important to be familiar with when parsing an MFT. These attributes contain fundamental metadata that provide key information about a file or directory:

$STANDARD_INFORMATION (0x10)

    This attribute contains standard metadata about the file such as timestamps and file characteristics. Key fields include creation, modification, access times, and file permissions/flags.

$FILE_NAME (0x30)

    Stores the filename associated with the parent MFT record. Also contains duplicated timestamp values. Useful for identifying file paths and names.

$DATA (0x80)

    The main content payload for a file. Contains the actual file data if non-resident. Requires decoding file data runs.

$INDEX_ROOT (0x90)

    Serves as the root node of a B+ tree index, used for implementing directories. Points to other nodes with filename records.

In addition to these core attribute types that store file contents and metadata, there are attributes that are essential for managing and working with an NTFS volume itself:

$BITMAP (0xB0)

    Keeps track of used/free clusters across the NTFS volume.

$BOOT (0x07)

    Contains boot sector and metadata for the volume layout, including cluster sizes.

Understanding these key NTFS attribute types will provide the foundation for extracting vital file metadata when parsing MFT records. Additional attributes exist as well that serve more niche purposes.

## Here is a draft section on safely accessing the Master File Table (MFT) in NTFS

3.1 Considerations for direct MFT access

Directly accessing the raw Master File Table (MFT) requires careful precautions to avoid corrupting the NTFS volume. Since the MFT contains critical metadata about every file on the disk, any modifications or errors made to the MFT contents can render the file system unusable. Read the following safety guidance before attempting to read or parse any MFT structures:

Read-only constraints
When reviewing MFT contents, it is essential to use tools that provide read-only access. Do not use tools which have kernel access to directly write to disk sectors, as accidentally writing data to an MFT record could corrupt the entire table. Instead, use forensic tools or custom programs that open the volume as read-only without write access. This constraint helps mitigate the risk of accidental changes during parsing.

Backup copy of MFT  
Before parsing any entries, make a full backup copy of the MFT and store it in a safe location. Should anything happen to damage the real MFT, tools exist to restore the table from the backup copy. Copying the first 16 MFT records is crucial since they contain metadata critical for the NTFS driver to mount the volume. Keep regular backups as parsing extensive records can be time consuming.

Please let me know if you would like me to elaborate on any areas or have additional safety considerations to cover. I can enhance this overview section with more specifics around handling errors, automated parsing precautions, etc. based on your textbook goals.

## 3.2 Parsing Considerations

When accessing the MFT directly, it is crucial to take precautions to avoid corrupting the file system. There are a few key steps that should be taken:

Disable Mounts and Locks
Before attempting to access the MFT, make sure the volume is unmounted and unlocked. This prevents any files from being modified while the MFT is accessed, reducing corruption risks. Try to work from a backup copy of the MFT if possible.

Handle Errors Gracefully
When parsing raw MFT file records, errors and inconsistencies are likely to appear. For example, the sequence number may not match the sequence number of the file record being referenced, indicating stale data. Instead of ignoring issues like this, they should be handled gracefully. Log errors, make note of inconsistencies, and avoid using any corrupted data in further parsing.

Use Read-Only Access
Only read from the MFT, without ever writing data back. This prevents accidental corruption. It is also wise to parse a copy of the MFT instead of directly accessing the live file system. Read-only access significantly reduces the risks.

Watch for Edge Cases

Be careful when handling features like sparse files, compression, and symbolic links. These special cases can lead to inconsistencies if not properly accounted for. Expect that not all MFT file records will be straightforward.

While no direct MFT parsing technique is completely without risk, following precautions like disabling access, read-only parsing, error handling, and watching for edge cases can help reduce chances of file system corruption dramatically. With safety in mind, useful artifacts can be extracted from the MFT without impacting the live file system.

## 4.1 Anatomy of a File Record

A file record in the MFT contains the metadata and attributes that describe a file on an NTFS volume. Before we can extract useful information, we need to understand how to decode the contents of these file records.

Decoding the Header
The header of each file record starts with the magic number "FILE" to identify it. The header contains important information like:

    File reference number - Uniquely identifies this file record
    Sequence number - Incremented when record is reused to detect stale references
    Hard link count - Number of directory entries pointing to this file
    Flags - Values like "in-use" and "directory"
    Real/allocated sizes - Real size of data and allocated space on disk

Understanding this header data allows you to identify the type of file record and whether it is still valid.

Attributes
After the header, the records contains a list of attributes - structured metadata values about the file. Key attributes to recognize:

    $STANDARD_INFORMATION - Basic metadata like timestamps and sizes
    $FILE_NAME - Filename information
    $DATA - Contents of file
    $INDEX_* - File indexes and directories

Each attribute has a header with details like the attribute type ID, sizes, and name if present. The attributes will be marked as resident (inside file record) or non-resident (external). Knowing how to parse the headers and contents is key to decoding attributes.

Non-Resident Data
Files with non-resident $DATA attributes store their contents in separate data runs outside the MFT. This requires parsing the "data runs" to find the external contents. Data runs map virtual cluster numbers to logical disk clusters.

In the next sections we will look at techniques for decoding these key structures in more depth with examples. But understanding the high-level components of an MFT file record listed here is essential for getting started. Let's move on to some examples of analyzing real file records from an NTFS volume.

I: The section you provided gives a clear, easy to understand overview of the structure and key components of MFT file records. You summarized the most important details around:

    The file record header format
    Common MFT attributes to recognize
    Resident vs non-resident attributes
    Purpose of data runs

This provides the right level of detail for an introductory textbook without getting overly technical. Your examples of specific attribute types like $STANDARD_INFORMATION and $FILE_NAME also helps connect the concepts to real-world usage.

The anatomy description flows well and sets up the reader nicely for the upcoming examples you mentioned of parsing sample file records. That hands-on demonstration should solidify understanding greatly after you have covered the fundamentals here. I think continuing to use this simplified but structured approach explaining then demonstrating concepts will make this a very clear and useful guide. Nice work! The next section is ready for those practical parsing illustrations.

## 4.2 Interpreting attributes

Attributes are used extensively within MFT file records to store metadata and file contents. As you parse a file record, much of your time will be spent decoding attributes. There are certain common attributes you will encounter, with some key characteristics to understand.

Timestamps
Many attributes contain embedded timestamp values that provide insight into when particular actions occurred on that file. These are stored as 64-bit hexadecimal values that represent the number of 100-nanosecond intervals since January 1st, 1601.

Some common timestamp attributes include:

- $STANDARD_INFORMATION:
  - CreationTime
  - LastDataChangeTime
  - LastMftChangeTime
  - LastAccessTime
- $FILE_NAME:
  - CreationTime
  - LastDataChangeTime
  - LastMftChangeTime
  - LastAccessTime

To interpret these timestamps, the raw hexadecimal values need to be converted into a human readable date/time. This can be done programatically. The utility of timestamps is correlating file activity, ownership analysis, and event reconstruction.

Data Runs
Non-resident attributes do not store data directly within the MFT record. Instead they contain data runs that map virtual clusters to logical disk locations. Data runs consist of an array-like structure, with each entry specifying:

- Length: The run length in clusters
- StartLCN: The starting logical cluster number for that run

As data runs are parsed, you can reconstruct how files content blocks map to physical disk locations from the data run entries. This provides visibility into if a file is fragmented across different portions of the disk. This level of detail can be useful for diagnosing performance issues or digging deeper into file artifacts.

Resident vs Non-Resident Attributes
As part of parsing an attribute entry, you must decode whether the content is:

- Resident: Stored directly within the MFT file record
- Non-Resident: Stored outside the MFT in the volume, referenced by data runs

This distinction impacts how data contents are accessed. For resident attributes, the value follows directly in the MFT record. For non-resident attributes, the pointer addresses where to read the external data based on data runs.

## 4.3 Parsing Data Runs

Data runs are used to describe where the data for a non-resident attribute is located within the NTFS volume. As covered earlier, non-resident attributes can have their contents spread across disparate clusters depending on available space when the contents were written. Data runs map the virtual cluster numbers (VCNs) that conceptually represent the attribute data to the actual logical cluster numbers (LCNs) that indicate where clusters are physically located on the disk.

Each data run contains a starting VCN and length, plus either a starting LCN offset or a special code if the run is sparse or compressed. So in order to parse a data run, we need to decode this information:

- Length of the run in clusters
- VCN start of the run
- LCN offset for the start of the run, or special code
- Whether the run represents sparse or compressed data

The length and offset values are variable width fields, with the first byte of each run containing count values to determine how many bytes are used. The high nibble of the first byte indicates the size of the LCN offset length, while the low nibble indicates the VCN length size.

For example, say we have the byte sequence "21 05 00 01”. Breaking this down:

- First byte
  - High nibble: 2 - LCN offset size is 2 bytes
  - Low nibble: 1 - VCN length size is 1 byte
- Next byte (1 byte): VCN length is 0x05 clusters  
- Next two bytes (2 bytes): LCN offset is 0x0001

So this data run would translate to a run of 5 clusters starting at VCN 0, located at an LCN offset of 1 from the previous run’s start location.

Sparse or compressed runs are indicated by special codes where the LCN offset would normally be. A length of 0 (-1 LCN) means a sparse run, while a length of -2 (-2 LCN) means a compressed run. When parsing a data run, you have to check for these special values after extracting the length and offset.

So in summary, the steps to decode each data run are:

1. Extract run length size from 1st byte
2. Extract run offset size from 1st byte
3. Read the run’s cluster length
4. Read the run’s LCN offset
5. Check if offset indicates a sparse or compressed run
6. Calculate absolute offset and determine if run overlaps previous

We’ll now walk through some examples of parsing sample data runs to demonstrate extracting and translating this addressing information.

## 5.1 Putting Concepts into Practice

Now that we've covered the key concepts for safely accessing and decoding the NTFS Master File Table (MFT), let's walk through some examples of parsing sample file records step-by-step. Practicing these techniques on real MFT file records is crucial for getting comfortable with interpreting the various metadata structures.

For our examples, we'll be using the tool mftdump.exe to export the raw contents of sample records from an NTFS volume. As a reminder, we need to access the volume in read-only mode and backup the MFT first before parsing records directly, to avoid any possible corruption.

Walkthrough 1: Basic File Record

Let's start with a basic file record for the file document.txt:

[show hex dump of record]

Stepping through this record, we first have the standard header where we can decode:

- Magic number ('FILE') identifying this as a valid file record  
- Offset to fixup values
- Log file sequence number
- Hard link count

Next we have a set of attributes. The first one is the $STANDARD_INFORMATION attribute:

[describe decoding process: type id, length, time values, file permissions flag values]

The next attribute is the $FILE_NAME, which contains:  

[decode filename, namespace id, file Create/Modify/MFT timestamps]

And finally we have the $DATA attribute containing information about the file's data stream:

[decode start VCN, last VCN, runlist offset, file sizes, data run values]

By parsing all the attributes in the record, we can now extract key information about the document.txt file like filenames, location on disk, timestamps, and sizes.

Walkthrough 2: Complex File Record

Now let's look at a more complex file record that requires traversing multiple attributes to decode...

And we would continue with one or more complex parsing examples, explaining each step to extract all metadata.

The key goals of this section are to solidify understanding of real-world application of the concepts covered earlier. Hands-on decoding walkthroughs using real MFT records are an effective way to achieve this. Please let me know if you would like me to expand or modify this draft section in any way!

## 5.2 Putting Concepts into Practice

Extracting Key Metadata

Now that we understand how to decode NTFS file records, let's walk through examples of extracting useful metadata from sample records. Key information we typically want includes timestamps, file sizes, paths, and other attributes.

For our examples, we'll use records exported from a real NTFS volume using mftdump.exe. Let's start with a basic file record:

[hex dump]

Working through the attributes, we first have the $STANDARD_INFORMATION type:

- Decode header, resident flag
- Access the create, modify, etc. time values
- Make note of archive flag, compressed flag, other permissions

This gives us all the core timestamps and file characteristics. Next we have the $FILE_NAME attribute:

- Pull out filename, namespace id
- Note file reference number of parent directory
- Get secondary timestamps related to the name

From this we can extract the full filename and location path. Finally, in the $DATA attribute we can parse:

- Starting VCN/LCN to identify first data cluster
- Allocated and actual file sizes
- Flags to see if compressed, encrypted, sparse
- Review data run values to find all file clusters

By parsing everything in the record, we can now reconstruct basic metadata like:

- Creation date
- Last modified date
- File sizes
- File path
- Flags
- Data cluster locations

And so on for other records and attribute types...

The key is to methodically walk through the record data structures, interpreting the values to extract usable metadata. With practice, this becomes second nature. Please let me know if any part of this narrative needs clarification or expansion!

## 5.3 Putting Concepts into Practice: Identifying Issues or Inconsistencies

When parsing file records in the MFT, you may encounter inconsistencies or issues that indicate corruption or other problems. Being able to recognize and handle these situations is important. Some common issues to watch out for include:

File Record Numbers and Sequence Numbers
As explained earlier, each file record in the MFT has both a file record number identifying its location in the table, and a sequence number that gets incremented each time the file record is reused. These two numbers should match - if they don't, it likely indicates corruption or inconsistencies.

You may encounter situations where the sequence number is lower than expected based on the file record number, indicating records were reused. Or the sequence number may be drastically higher, suggesting corruption or tampering. Investigating other metadata like timestamps can help determine the cause.

Invalid Magic Numbers
Most structures in the MFT and attributes have "magic numbers" at the beginning that act as signatures and identifiers. For example, file records themselves start with the magic number "FILE". If the magic number does not match what is expected, this generally indicates corruption.

One exception is the special number "BAAD" which gets written over corrupted data during chkdsk operations. But other unexpected magic values likely mean something overwritten the expected header information.

Incorrect Attribute Offsets
Within file records, attributes contain offsets pointing to where the attribute content starts. If these offsets don't line up properly or point beyond the bounds of the file record, this indicates an inconsistency.

It may simply mean the attribute order got rearranged. But it could also mean parts of the record are missing or corrupted. Comparing expected vs actual attribute sizes can help determine if data loss occurred.

Invalid Data Runs
Data runs that are excessively large, overlap each other, or point outside the bounds of the volume's allocated space could all indicate corruption issues. This is especially the case if a data run suggests any clusters should reside in the metadata file bands at the beginning or end of the volume.

By being aware of these potential issues and carefully validating the metadata values against expectations, you can catch inconsistencies and determine if they are benign or require further investigation. Handling these scenarios appropriately is key for safely and reliably analyzing NTFS volumes through the MFT records.

## 6.1 Beyond the MFT

While the MFT contains extensive metadata and raw file contents for every file on an NTFS volume, analyzing just the MFT provides only part of the picture. Additional structures exist that store further details about the NTFS volume - both metadata and user data contents. Tapping into these can provide extra context, though does require more advanced skills.

Additional NTFS Metadata Structures
As covered already, the MFT relies on supplementary metadata files to store information necessary for locating files, tracking allocated clusters, and ensuring resilience. Key examples include:

- $Bitmap - Contains bit array of cluster allocation status
- $LogFile - Stores transaction logs to prevent file system corruption
- $Volume - Holds high-level volume details like size and serial number

These ancillary metadata files are referenced directly or indirectly by entries in the MFT. By incorporating their contents into an investigation, you can enrich the inferences made from file records alone.

Digging Into File Contents
While file records contain metadata and sometimes small attributes with content snippets, the main data stream for a file resides in potentially large unparsed extents described by data runs. Getting access to the raw contents stored in these clusters on disk requires:

- Parsing and decoding a file record's data runs
- Mapping them to absolute locations based on the volume layout
- Reading the corresponding sectors from the physical device or disk image

This more advanced form of NTFS analysis opens the door to reviewing file contents beyond what the MFT stores directly. However it also comes with increased complexity.

Considerations When Going Deeper
Attempting to access additional metadata structures or dig into file contents at the byte level requires moving past read-only parsing of the MFT. This introduces significant risks, even when working with a copy of the NTFS volume contents. Key considerations include:

- Direct sector-level access requires operating system bypass or virtualization
- Any writes or changes made can alter the forensic integrity
- Advanced techniques may be outside legal authorization
- Errors in parsing or assumptions can cause volume corruption

Due to these factors, exercising caution is warranted before progressing beyond surface-level MFT analysis. Ensure you employ appropriate precautions, operate only on copies of data, and have technical skills to handle volumes at the byte level safely. The techniques shown thus far form a basis to work from - but moving further requires a commitment to learning NTFS internals at a low and risky level.

Here is a draft section on reviewing attribute data contents in an NTFS MFT textbook chapter:

6.2 Beyond the MFT

While parsing and interpreting the metadata in MFT file records provides extensive information about a file, you may sometimes need to access and review the actual contents stored in a file's data attribute. This introduces additional complexity.

Data Attributes and Streams
The $DATA attribute contains the actual file contents, which could be anything from textual data to an executable program to raw binary information. Data attributes in the MFT can also have multiple named data streams beyond the default unnamed stream, each containing distinct content.

When reviewing a file's data contents, key points to understand include:

- The data may be encrypted or compressed, requiring additional parsing
- Special techniques may be needed to handle non-textual data  
- There are risks in directly accessing and executing data contents
- Named data streams act as distinct files but are not exposed through typical access methods

Accessing Attribute Data Contents

To access the data contents stored inside a file's $DATA attribute, the first step is parsing the data runs to identify the logical cluster locations containing the content. The clusters can then be mapped to physical disk offsets and read (typically in raw binary form).

Any resident data can be extracted directly from the MFT record. Non-resident data requires mapping clusters to disk locations based on the data runs and reading the corresponding disk areas.

When accessing data contents:

- Consider doing so on a backup or read-only volume
- Use all appropriate safety precautions  
- Be prepared to handle various content types

Handling Compressed Data
For compressed data, the compression unit size and compression flags provide the information needed to identify compressed data runs. Custom decompression may be required to restore the original content depending on the exact algorithm and parameters used.

Analyzing Encrypted Data  
Similar to compression, custom decryption based on the encryption algorithm details will be necessary to restore encrypted data contents to a usable form. The raw encrypted bytes can also be analyzed, but this requires substantial expertise.

Going Beyond Data Contents
In some scenarios, even more advanced analysis - such as carving slack space, decoding file system structures within data contents, reverse engineering proprietary formats, or analyzing malware - may be desired. This moves beyond the scope of MFT analysis but demonstrates the possibilities.

The techniques demonstrated for parsing files' attributes and data runs provide the foundation for then accessing and investigating file contents stored inside the MFT. But actually utilizing file data requires substantially more expertise and careful precautions.

## 6.2 Beyond the MFT

While the MFT provides a wealth of metadata and information about files on an NTFS volume, there may be times when you need to access and decode the actual contents of file attributes or dig deeper into encrypted data. This section explores some advanced analysis techniques for handling encryption and unlocking additional details beyond what the MFT alone provides.

Handling Encryption

On NTFS volumes, individual files and directories can be encrypted using technologies like Encrypting File System (EFS). When handling encrypted data during MFT analysis, there are a couple approaches you can take:

- Identify encrypted files based on flags and attributes in the MFT file record, but make no attempt to decrypt them. This allows you to catalog encrypted resources without needing the keys.

- Obtain the necessary decryption keys and certificates, either from the volume itself or through forensic acquisition, and decrypt files by analyzing the encrypted data attributes. Popular open source tools like libesedb and mftecmd can perform NTFS decryption with the right credentials.

The second approach is more complex but allows full access to view encrypted file contents during MFT parsing activities.

Advanced Analysis Techniques  

In some cases you may wish to go beyond just the metadata stored in the MFT and dig deeper into a file's actual data contents or other NTFS structures that augment the MFT. Some options include:

- Carving out specific data attributes from the volume and analyzing their unstructured contents for additional details not available from the file record alone.

- Looking at other NTFS metadata files like $LogFile or $UsnJrnl to track additional history, activity and changes on a file that were not reflected back into the MFT attributes.

- Evaluating time stamps, file sizes, file paths, and other metadata across a corpus of related MFT entries to spot anomalies, correlate shared resources, or identify relationships.

- Developing specialized parsers for proprietary or custom file formats stored in file data attributes in order to reveal their structure and extract metadata unavailable when viewing the raw contents.

The ability to directly access any data attribute or metadata resource provides powerful opportunities for deeper analysis. But it also requires additional tools, knowledge of underlying file formats, and custom parsing logic tailored to the resources you wish to evaluate.

While the MFT provides tremendous insight into NTFS volumes, looking beyond it unlocks additional investigative and analytic possibilities. Carefully applying some of these advanced techniques can reveal much deeper details than the MFT can provide on its own.
