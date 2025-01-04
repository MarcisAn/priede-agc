use crate::OPTCODE;

fn format_bytecode_line(optcode: OPTCODE, counter: usize) -> String {
    match optcode {
        OPTCODE::Display { display, register } =>
            format!("	CA PR{}\n	TC {}", register, match display {
                crate::DisplayOptions::Prog => "DSPPROG",
                crate::DisplayOptions::Verb => todo!(),
                crate::DisplayOptions::Noun => todo!(),
                crate::DisplayOptions::R1 => todo!(),
                crate::DisplayOptions::R2 => todo!(),
                crate::DisplayOptions::R3 => todo!(),
            }),
        OPTCODE::LoadNumber { const_num, register } =>
            format!("	CA PC{}\n	TS PR{}", const_num, register),
        OPTCODE::Add { target_register, value_register } =>
            format!("	CA PR{}\n	AD PR{}\n	TS PR{}", target_register, value_register, target_register),
        OPTCODE::Subtract { target_register, value_register } =>
            format!(
                "	CA PR{}\n	EXTEND\n	SU PR{}\n	TS PR{}",
                target_register,
                value_register,
                target_register
            ),
        OPTCODE::Multiply { target_register, value_register } =>
            format!(
                "	CA PR{}\n	EXTEND\n	MP PR{}\n	LXCH A\n	TS PR{}",
                target_register,
                value_register,
                target_register
            ),
        OPTCODE::Divide { target_register, value_register } =>
            format!(
                "	EXTEND\n	DCA PR{}\n	EXTEND\n	DV PR{}\n	TS PR{}",
                target_register,
                value_register,
                target_register
            ),
        OPTCODE::JumpIfZero { target, register } =>
            format!("	CA PR{}\n	EXTEND\n	BZF JUMT{}", register, target),
        OPTCODE::JumpTarget { number } => format!("JUMT{}", number),
        OPTCODE::Jump { target } => format!("	TC JUMT{}", target),
        OPTCODE::LoadInA { register } => todo!(),
        OPTCODE::AreEqual { target_a_register, b_register, comparison_id } =>
            format!(
                "	CA PR{}\n	EXTEND\n	SU PR{}\n	TS L\n	CA ONE\n	TS PR{}\n	CA L\n	EXTEND\n	BZF COMPJ{}\n	CS ZERO\n	TS PR{}\nCOMPJ{}",
                target_a_register,
                b_register,
                target_a_register,
                comparison_id,
                target_a_register,
                comparison_id
            ),
        OPTCODE::LargerEqual { target_a_register, b_register, comparison_id } =>
            format!(
                "	CA PR{}\n	EXTEND\n	SU PR{}\n	TS L\n	CA ONE\n	TS PR{}\n	CA L\n	EXTEND\n	BZMF COMPJ{}\n	CS ZERO\n	TS PR{}\nCOMPJ{}",
                target_a_register,
                b_register,
                target_a_register,
                comparison_id,
                target_a_register,
                comparison_id
            ),
        OPTCODE::LargerThan { target_a_register, b_register, comparison_id } =>
            format!(
                "	CA PR{}\n	EXTEND\n	SU PR{}\n	TS L\n	INCR L\n	CA ONE\n	TS PR{}\n	CA L\n	EXTEND\n	BZMF COMPJ{}\n	CS ZERO\n	TS PR{}\nCOMPJ{}",
                target_a_register,
                b_register,
                target_a_register,
                comparison_id,
                target_a_register,
                comparison_id
            ),
        OPTCODE::LessEqual { target_a_register, b_register, comparison_id } =>
            format!(
                "	CA PR{}\n	EXTEND\n	SU PR{}\n	TS L\n	CA ONE\n	TS PR{}\n	CA L\n	EXTEND\n	BZMF COMPJ{}\n	CS ZERO\n	TS PR{}\nCOMPJ{}",
                b_register,
                target_a_register,
                target_a_register,
                comparison_id,
                target_a_register,
                comparison_id
            ),
        OPTCODE::LessThan { target_a_register, b_register, comparison_id } =>
            format!(
                "	CA PR{}\n	EXTEND\n	SU PR{}\n	TS L\n	INCR L\n	CA ONE\n	TS PR{}\n	CA L\n	EXTEND\n	BZMF COMPJ{}\n	CS ZERO\n	TS PR{}\nCOMPJ{}",
                b_register,
                target_a_register,
                target_a_register,
                comparison_id,
                target_a_register,
                comparison_id
            ),
        OPTCODE::LoadVariable {  target_reg, id } => format!("	CA PV{}\n	TS PR{}", id, target_reg),
        OPTCODE::SetVariable { source_reg, id } => format!("	CA PR{}\n	TS PV{}", source_reg, id),
    }
}

pub fn format_asm(bytecode: Vec<OPTCODE>, constants: Vec<String>, register_count: usize, variable_count: usize) -> String {
    let mut main_code = String::new();
    let mut constant_def_asm: String = String::new();
    let mut register_definitions = String::new();

    let mut counter = 0;
    for optcode in bytecode {
        main_code += &format_bytecode_line(optcode, counter);
        main_code += "\n\n";
        counter += 1;
    }

    for (i, constant) in constants.into_iter().enumerate() {
        constant_def_asm += &format!("PC{} DEC {}\n", i, constant);
    }

    for i in 0..register_count {
        register_definitions += &format!("PR{} ERASE", i);
        register_definitions += "\n";
    }

	for i in 0..variable_count {
        register_definitions += &format!("PV{} ERASE", i);
        register_definitions += "\n";
    }

    println!("{}", constant_def_asm);
    println!("{}", register_definitions);

    println!("{}", main_code);

    format!(
        "
A               		EQUALS          0
L               		EQUALS          1               # L AND Q ARE BOTH CHANNELS AND REGISTERS.
Q               		EQUALS          2
EBANK           		EQUALS          3
FBANK           		EQUALS          4
Z               		EQUALS          5               # ADJACENT TO FBANK AND BBANK FOR DXCH Z
BBANK           		EQUALS          6               # (DTCB) AND DXCH FBANK (DTCF).
                                                        	# REGISTER 7 IS A ZERO-SOURCE, USED BY ZL.
ARUPT           		EQUALS          10              # INTERRUPT STORAGE.
LRUPT           		EQUALS          11
QRUPT           		EQUALS          12
SAMPTIME        		EQUALS          13              # SAMPLED TIME 1 & 2.
ZRUPT           		EQUALS          15              # (13 AND 14 ARE SPARES.)
BANKRUPT        		EQUALS          16              # USUALLY HOLDS FBANK OR BBANK.
BRUPT           		EQUALS          17              # RESUME ADDRESS AS WELL.
CYR             		EQUALS          20
SR              		EQUALS          21
CYL             		EQUALS          22
EDOP            		EQUALS          23              # EDITS INTERPRETIVE OPERATION CODE PAIRS.
TIME2           		EQUALS          24
TIME1           		EQUALS          25
TIME3           		EQUALS          26
TIME4           		EQUALS          27
TIME5           		EQUALS          30
TIME6           		EQUALS          31


                                SETLOC          67
NEWJOB                          ERASE                           # Allocate a variable at the location checked by the Night Watchman.

# More variables.
KEYBUF                          ERASE                           # 040 when empty, 0-037 when holding a keycode
LAST040                         ERASE                           # Most recent value from input channel 040.
SECOND                          ERASE                           # Storage for components of time
MINUTE                          ERASE
HOUR                            ERASE
DAY                             ERASE
MONTH                           ERASE
YEAR                            ERASE
SIGN				ERASE				# For buffering sign+digits in conversion of integer to decimal string.
DIGIT1				ERASE
DIGIT2				ERASE
DIGIT3				ERASE
DIGIT4				ERASE
DIGIT5				ERASE
SIGNA				ERASE				# For buffering sign+digits in conversion of integer to decimal string.
DIGIT1A				ERASE
DIGIT2A				ERASE
DIGIT3A				ERASE
DIGIT4A				ERASE
DIGIT5A				ERASE
DSPR				ERASE				# Return address for DSPxxxx functions.
DIGIT25				ERASE
DIGIT31				ERASE
DIVISOR				ERASE				# A dummy variable used to store divisors.
DSPMODE				ERASE				# display-mode: 0 time, 1 acceleration, 2 magnetometer, etc.
LLLOW				ERASE				# LS word of lat or lon
LLHIGH				ERASE				# MS word of lat or lon.
LLRET				ERASE				# Return address for LL2.

# PRIEDE VARIABLES
{}

                                SETLOC          4000            # The interrupt-vector table.

                                                                # Come here at power-up or GOJAM
                                INHINT                          # Disable interrupts for a moment.
                                                                # Set up the TIME3 interrupt, T3RUPT.  TIME3 is a 15-bit
                                                                # register at address 026, which automatically increments every
                                                                # 10 ms, and a T3RUPT interrupt occurs when the timer
                                                                # overflows.  Thus if it is initially loaded with 037774,
                                                                # and overflows when it hits 040000, then it will
                                                                # interrupt after 40 ms.
                                CA              O37774
                                TS              TIME3
                                TCF             STARTUP         # Go to your real code.

                                RESUME                          # T6RUPT
                                NOOP
                                NOOP
                                NOOP

                                RESUME                          # T5RUPT
                                NOOP
                                NOOP
                                NOOP

                                DXCH            ARUPT           # T3RUPT
                                EXTEND                          # Back up A, L, and Q registers
                                QXCH            QRUPT
                                TCF             T3RUPT

                                RESUME                          # T4RUPT
                                NOOP
                                NOOP
                                NOOP

                                DXCH            ARUPT           # KEYRUPT1
                                EXTEND                          # Back up A, L, and Q registers
                                QXCH            QRUPT
                                TCF             KEYRUPT

                                DXCH            ARUPT           # KEYRUPT2
                                EXTEND                          # Back up A, L, and Q registers
                                QXCH            QRUPT
                                TCF             KEYRUPT

                                RESUME                          # UPRUPT
                                NOOP
                                NOOP
                                NOOP

                                RESUME                          # DOWNRUPT
                                NOOP
                                NOOP
                                NOOP

                                RESUME                          # RADAR RUPT
                                NOOP
                                NOOP
                                NOOP

                                RESUME                          # RUPT10
                                NOOP
                                NOOP
                                NOOP

# The interrupt-service routine for the TIME3 interrupt every 40 ms.
T3RUPT                          CAF             O37774          # Schedule another TIME3 interrupt in 40 ms.
                                TS              TIME3

                                                                # And resume the main program
                                DXCH            ARUPT           # Restore A, L, and Q, and exit the interrupt
                                EXTEND
                                QXCH            QRUPT
                                RESUME

# Interrupt-service code for DSKY keycode
KEYRUPT                         EXTEND
                                READ            15              # Read the DSKY keycode input channel
                                MASK            O37             # Get rid of all but lowest 5 bits.
                                TS              KEYBUF          # Save the keycode for later.
                                CA              ZERO            # Clear the input channel.
                                EXTEND
                                WRITE           15
                                DXCH            ARUPT           # Restore A, L, and Q, and exit the interrupt
                                EXTEND
                                QXCH            QRUPT
                                RESUME

STARTUP                         RELINT                          # Reenable interrupts.

	TC BLANKALL
{}
MAINLOOP                        CS              NEWJOB          # Tickle the Night Watchman.
								TCF		MAINLOOP



#######################################################################################################################################
# This ends the main loop, which continues forever, so we can put other functions
# below this point.

# Blank all of the numerical displays.
BLANKALL 			CA		OPCODEP
				EXTEND
				WRITE		10
				CA		OPCODEV
				EXTEND
				WRITE		10
				CA		OPCODEN
				EXTEND
				WRITE		10
				CA		OPCODR11
				EXTEND
				WRITE		10
				CA		OPCDR123
				EXTEND
				WRITE		10
				CA		OPCDR145
				EXTEND
				WRITE		10
				CA		OPCDR212
				EXTEND
				WRITE		10
				CA		OPCDR234
				EXTEND
				WRITE		10
				CA		OPR25R31
				EXTEND
				WRITE		10
				CA		OPCDR323
				EXTEND
				WRITE		10
				CA		OPCDR345
				EXTEND
				WRITE		10
				CA		ZERO
				TS		DIGIT31
				TS		DIGIT25
				RETURN

# Convert an integer in the accumulator to SIGN/DIGIT1/.../DIGIT5.  At the end, all 5 DIGITx
# variables will have values from 0-9, and SIGN will be 0 for + or 1 for -.  Note: calls no
# other routines.
CONV10				TS		L		# Save the argument
				EXTEND
				BZF		CONV10Z		# Argument is 0.
				EXTEND
				BZMF		CONV10M		# Argument is negative, we'll need to invert
CONV10Z				CA		ZERO		# Record the sign as positive (0 -> SIGN).
				TS		SIGN
CONV10P				CA		TEN
				TS		DIVISOR
				CA		ZERO		# Note that L still contains the original argument.
				EXTEND
				DV		DIVISOR
				LXCH		A		# Save remainder as digit 5 and put quotient into L.
				TS		DIGIT5
				CA		ZERO
				EXTEND
				DV		DIVISOR
				LXCH		A
				TS		DIGIT4
				CA		ZERO
				EXTEND
				DV		DIVISOR
				LXCH		A
				TS		DIGIT3
				CA		ZERO
				EXTEND
				DV		DIVISOR
				LXCH		A
				TS		DIGIT2
				CA		ZERO
				EXTEND
				DV		DIVISOR
				LXCH		A
				TS		DIGIT1
				RETURN

# Lat an lon are each input on two input channels, a more-significant one containing the integer part of
# the value times 10, and a less-significant on containing the fractional part of the value times 10, times
# 10000.  The LL2 function assumes that those values have been stored in variables called LLHIGH and LLLOW,
# respectively, and create SIGN, DIGIT1, ..., DIGIT5 containing the value in the form +/- XXX.XX.
LL2 				EXTEND
				QXCH		LLRET
				CA		LLLOW		# Process LS word
				TCR		CONV10		# Convert to (SIGN, DIGIT1, ..., DIGIT5)
				TCR		PATT5DIG
				TCR		DIG2A		# Move SIGN/DIGITx to SIGNA/DIGITxA
				CA		LLHIGH		# Process MS word
				TCR		CONV10		# Convert to SIGN/DIGITx
				TCR		PATT5DIG
				CA		DIGIT2		# Rearrange the DIGITx values somewhat. (Multiply by 10.)
				TS		DIGIT1
				CA		DIGIT3
				TS		DIGIT2
				CA		DIGIT4
				TS		DIGIT3
				CA		DIGIT5
				TS		DIGIT4
				CA		DIGIT2A
				TS		DIGIT5
				EXTEND
				QXCH		LLRET
				RETURN
# Similarly, the LL5 function puts the integral lat or lon in SIGN,DIGITx and 5 decimal places in SIGNA,DIGITxA.
LL5 				EXTEND
				QXCH		LLRET
				CA		LLLOW		# Process LS word
				TCR		CONV10		# Convert to (SIGN, DIGIT1, ..., DIGIT5)
				TCR		PATT5DIG
				TCR		DIG2A		# Move SIGN/DIGITx to SIGNA/DIGITxA
				CA		LLHIGH		# Process MS word
				TCR		CONV10		# Convert to SIGN/DIGITx
				TCR		PATT5DIG
				CA		DIGIT5		# Rearrange the DIGITx values somewhat. (Divide by 10.)
				TS		DIGIT1A
				CA		DIGIT4
				TS		DIGIT5
				CA		DIGIT3
				TS		DIGIT4
				CA		DIGIT2
				TS		DIGIT3
				CA		DIGIT1
				TS		DIGIT2
				EXTEND
				QXCH		LLRET
				RETURN

# Transfer set of SIGN/DIGITx to SIGNA/DIGITxA
DIG2A				CA		SIGN
				TS		SIGNA
				CA		DIGIT1
				TS		DIGIT1A
				CA		DIGIT2
				TS		DIGIT2A
				CA		DIGIT3
				TS		DIGIT3A
				CA		DIGIT4
				TS		DIGIT4A
				CA		DIGIT5
				TS		DIGIT5A
				RETURN
# ... and vice-versa.
A2DIG				CA		SIGNA
				TS		SIGN
				CA		DIGIT1A
				TS		DIGIT1
				CA		DIGIT2A
				TS		DIGIT2
				CA		DIGIT3A
				TS		DIGIT3
				CA		DIGIT4A
				TS		DIGIT4
				CA		DIGIT5A
				TS		DIGIT5
				RETURN

# The argument was negative.  Must invert.
CONV10M				CA		ONE		# Record the sign as negative (1 -> SIGN).
				TS		SIGN
				CA		L		# Invert the argument to make it positive.
				COM
				TS		L
				TCF		CONV10P		# Go back to the processing for positive numbers.

# Packs two digits previously formed by CONV10 into a suitable
# form for output.  The channel 10 opcode has to be added in afterward.
# Suitable for PROG, VERB, and NOUN areas.  Result returned in A.
# Note: calls no other routines.
PACK2DIG			CA		DIGIT5
				INDEX		A
				CA		DIGPATTS
				TS		DIGIT5
				CA		DIGIT4
				INDEX		A
				CA		DIGPATTS
				TS		DIGIT4
				EXTEND
				MP		D32		# Shift by 5 places.
				CA		DIGIT5
				AD		L
				RETURN

# Convert DIGIT1 ... DIGIT5 to their DSKY patterns.  Note: calls no other
# routines.
PATT5DIG			CA		DIGIT1
				INDEX		A
				CA		DIGPATTS
				TS		DIGIT1
				CA		DIGIT2
				INDEX		A
				CA		DIGPATTS
				TS		DIGIT2
				CA		DIGIT3
				INDEX		A
				CA		DIGPATTS
				TS		DIGIT3
				CA		DIGIT4
				INDEX		A
				CA		DIGPATTS
				TS		DIGIT4
				CA		DIGIT5
				INDEX		A
				CA		DIGPATTS
				TS		DIGIT5
				RETURN

# Display the number in the accumulator as decimal in DSKY PROG
DSPPROG				EXTEND
				QXCH		DSPR
				TCR		CONV10
				TCR		PACK2DIG
				AD		OPCODEP
				EXTEND
				WRITE		10
				EXTEND
				QXCH		DSPR
				RETURN

# Display the number in the accumulator as decimal in DSKY VERB
DSPVERB				EXTEND
				QXCH		DSPR
				TCR		CONV10
				TCR		PACK2DIG
				AD		OPCODEV
				EXTEND
				WRITE		10
				EXTEND
				QXCH		DSPR
				RETURN

# Display the number in the accumulator as decimal in DSKY NOUN
DSPNOUN				EXTEND
				QXCH		DSPR
				TCR		CONV10
				TCR		PACK2DIG
				AD		OPCODEN
				EXTEND
				WRITE		10
				EXTEND
				QXCH		DSPR
				RETURN

# Display the number in the accumulator as decimal in DSKY R1.
DSPR1				EXTEND
				QXCH		DSPR
				TCR		CONV10
				TCR		PATT5DIG
				EXTEND
				QXCH		DSPR
				# Fall through to DIG2R1.

# Display the number in SIGN/DIGITx in DSKY R1
DIG2R1				EXTEND
				QXCH		DSPR
				CA		DIGIT1		# First write DIGIT1
				AD		OPCODR11
				EXTEND
				WRITE		10
				CA		DIGIT2		# Next, DIGIT2 and DIGIT3
				EXTEND
				MP		D32
				CA		DIGIT3
				ADS		L
				CA		OPCDR123
				ADS		L
				CA		SIGN
				EXTEND
				BZF		DSPR1POS
				TCF		DSPR1NPS
DSPR1POS			CA		SIGNBIT
				ADS		L
DSPR1NPS			CA		L
				EXTEND
				WRITE		10
				CA		DIGIT4		# And finally, DIGIT4 and DIGIT5
				EXTEND
				MP		D32
				CA		DIGIT5
				ADS		L
				CA		OPCDR145
				ADS		L
				CA		SIGN
				EXTEND
				BZF		DSPR1NNG
				CA		SIGNBIT
				ADS		L
DSPR1NNG			CA		L
				EXTEND
				WRITE		10
				EXTEND
				QXCH		DSPR
				RETURN

# Display the number in the accumulator as decimal in DSKY R2
DSPR2				EXTEND
				QXCH		DSPR
				TCR		CONV10
				TCR		PATT5DIG
				EXTEND
				QXCH		DSPR
				# Fall through to DIG2R2.

# Display the number in SIGN/DIGITx in DSKY R2
DIG2R2				EXTEND
				QXCH		DSPR
				CA		DIGIT5		# Save DIGIT5 for DSPR3 routine.
				TS		DIGIT25
				CA		DIGIT1		# First, DIGIT1 and DIGIT2
				EXTEND
				MP		D32
				CA		DIGIT2
				ADS		L
				CA		OPCDR212
				ADS		L
				CA		SIGN
				EXTEND
				BZF		DSPR2POS
				TCF		DSPR2NPS
DSPR2POS			CA		SIGNBIT
				ADS		L
DSPR2NPS			CA		L
				EXTEND
				WRITE		10
				CA		DIGIT3		# Next, DIGIT3 and DIGIT4
				EXTEND
				MP		D32
				CA		DIGIT4
				ADS		L
				CA		OPCDR234
				ADS		L
				CA		SIGN
				EXTEND
				BZF		DSPR2NNG
				CA		SIGNBIT
				ADS		L
DSPR2NNG			CA		L
				EXTEND
				WRITE		10
				CA		DIGIT5		# Finally, DIGIT5 (and R3 DIGIT1)
				EXTEND
				MP		D32
				CA		DIGIT31
				AD		L
				AD		OPR25R31
				EXTEND
				WRITE		10
				EXTEND
				QXCH		DSPR
				RETURN

# Display the number in the accumulator as decimal in DSKY R3
DSPR3				EXTEND
				QXCH		DSPR
				TCR		CONV10
				TCR		PATT5DIG
				EXTEND
				QXCH		DSPR
				# Fall through to DIG2R3.

# Display the number in SIGN/DIGITx in DSKY R3
DIG2R3				EXTEND
				QXCH		DSPR
				CA		DIGIT1		# Save DIGIT1 for DSPR2 routine.
				TS		DIGIT31
				CA		DIGIT25		# First, (R2 DIGIT5 and) DIGIT1
				EXTEND
				MP		D32
				CA		DIGIT1
				AD		L
				AD		OPR25R31
				EXTEND
				WRITE		10
				CA		DIGIT2		# Next, DIGIT2 and DIGIT3
				EXTEND
				MP		D32
				CA		DIGIT3
				ADS		L
				CA		OPCDR323
				ADS		L
				CA		SIGN
				EXTEND
				BZF		DSPR3POS
				TCF		DSPR3NPS
DSPR3POS			CA		SIGNBIT
				ADS		L
DSPR3NPS			CA		L
				EXTEND
				WRITE		10
				CA		DIGIT4		# And finally, DIGIT4 and DIGIT5
				EXTEND
				MP		D32
				CA		DIGIT5
				ADS		L
				CA		OPCDR345
				ADS		L
				CA		SIGN
				EXTEND
				BZF		DSPR3NNG
				CA		SIGNBIT
				ADS		L
DSPR3NNG			CA		L
				EXTEND
				WRITE		10
				EXTEND
				QXCH		DSPR
				RETURN

# Display the number in YEAR as a year in DSKY R3.  (Similar to
# DSPR3, except no sign bit, and first digit is blank.)
DSPR3Y				EXTEND
				QXCH		DSPR
				CA		YEAR
				TCR		CONV10
				TCR		PATT5DIG
				CA		ZERO		# Save first digit (a blank) for DSPR2 routine.
				TS		DIGIT31

				CA		DIGIT25		# First, (R2 DIGIT5 and) DIGIT1 (blank)
				EXTEND
				MP		D32
				CA		ZERO
				AD		L
				AD		OPR25R31
				EXTEND
				WRITE		10
				CA		DIGIT2		# Next, DIGIT2 and DIGIT3
				EXTEND
				MP		D32
				CA		DIGIT3
				AD		L
				AD		OPCDR323
				EXTEND
				WRITE		10

				CA		DIGIT4		# And finally, DIGIT4 and DIGIT5
				EXTEND
				MP		D32
				CA		DIGIT5
				AD		L
				AD		OPCDR345
				EXTEND
				WRITE		10

				EXTEND
				QXCH		DSPR
				RETURN

# Display the numbers in MONTH and DAY as two 2-digit fields in DSKY R2.
DSPR2MD				EXTEND
				QXCH		DSPR

				CA		MONTH
				TCR		CONV10
				TCR		PACK2DIG
				AD		OPCDR212
				EXTEND
				WRITE		10

				CA		DAY
				TCR		CONV10
				CA		DIGIT5
				INDEX		A
				CA		DIGPATTS
				TS		DIGIT5
				CA		DIGIT4
				INDEX		A
				CA		DIGPATTS
				AD		OPCDR234
				EXTEND
				WRITE		10

				CA		DIGIT5
				TS		DIGIT25
				EXTEND
				MP		D32
				CA		L
				AD		DIGIT31
				AD		OPR25R31
				EXTEND
				WRITE		10

				EXTEND
				QXCH		DSPR
				RETURN

# Display the numbers in HOUR and MINUTE as two 2-digit fields in DSKY R1.
DSPR1HM				EXTEND
				QXCH		DSPR

				CA		HOUR
				TCR		CONV10

				CA		DIGIT4
				INDEX		A
				CA		DIGPATTS
				AD		OPCODR11
				EXTEND
				WRITE		10

				CA		DIGIT5
				INDEX		A
				CA		DIGPATTS
				EXTEND
				MP		D32
				CA		L
				AD		OPCDR123
				EXTEND
				WRITE		10

				CA		MINUTE
				TCR		CONV10
				TCR		PACK2DIG
				AD		OPCDR145
				EXTEND
				WRITE		10

				EXTEND
				QXCH		DSPR
				RETURN

# Define any constants that are needed.

#PRIEDE CONSTANTS
{}

O37774                          OCT             37774
ZERO                            OCT             0
ONE				OCT		1
TWO                             OCT             2
FOUR                            OCT             4
EIGHT				DEC		8
MINUS1				DEC		-1
MINUS8				DEC		-8
TEN				DEC		10
O37                             OCT             37              # Mask with lowest 5 bits set.
O77                             OCT             77              # Mask with lowest 6 bits set.
NOKEY                           OCT             40
D32				DEC		32
D64				DEC		64
D1024				DEC		1024
OPCODEP				DEC		11 B11
OPCODEV				DEC		10 B11
OPCODEN				DEC		9 B11
OPCODR11			DEC		8 B11
OPCDR123			DEC		7 B11
OPCDR145			DEC		6 B11
OPCDR212			DEC		5 B11
OPCDR234			DEC		4 B11
OPR25R31			DEC		3 B11
OPCDR323			DEC		2 B11
OPCDR345			DEC		1 B11
SIGNBIT				DEC		1 B10
# DSKY digit patterns for the digit 0-9.
DIGPATTS			DEC		21
				DEC		3
				DEC		25
				DEC		27
				DEC		15
				DEC		30
				DEC		28
				DEC		19
				DEC		29
				DEC		31


",
        register_definitions,
        main_code,
        constant_def_asm
    )
}
