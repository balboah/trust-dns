// Copyright 2015-2017 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(missing_docs)]

use std::io;

#[cfg(feature = "openssl")]
use openssl::error::ErrorStack as SslErrorStack;
#[cfg(not(feature = "openssl"))]
use self::not_openssl::SslErrorStack;


error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    types {
        ProtoError, ProtoErrorKind, ProtoChainErr, ProtoResult;
    }

    // Automatic conversions between this error chain and other
    // error chains. In this case, it will e.g. generate an
    // `ErrorKind` variant called `Dist` which in turn contains
    // the `rustup_dist::ErrorKind`, with conversions from
    // `rustup_dist::Error`.
    //
    // This section can be empty.
    links {
    //   super::decode_error::Error, super::decode_error::ErrorKind, Decode;
    //   super::encode_error::Error, super::encode_error::ErrorKind, Encode;
    //   DnsSecError, ProtoErrorKind, DnsSec;
    }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // boxed as the error cause and wrapped in a new error with,
    // in this case, the `ErrorKind::Temp` variant.
    //
    // This section can be empty.
    foreign_links {
      ::std::io::Error, Io, "io error";
      ::std::net::AddrParseError, AddrParseError, "network address parse error";
      ::std::num::ParseIntError, ParseIntError, "error parsing number";
      ::std::string::FromUtf8Error, FromUtf8Error, "utf8 conversion error";
      SslErrorStack, SSL, "ssl error";
    }

    // Define additional `ErrorKind` variants. The syntax here is
    // the same as `quick_error!`, but the `from()` and `cause()`
    // syntax is not supported.
    errors {
      Canceled(c: ::futures::sync::oneshot::Canceled) {
        description("future was canceled")
        display("future was canceled: {:?}", c)
      }

      CharacterDataTooLong(len: usize) {
        description("char data length exceeds 255")
        display("char data length exceeds 255: {}", len)
      }

      DnsKeyProtocolNot3(value: u8) {
        description("dns key value unknown, must be 3")
        display("dns key value unknown, must be 3: {}", value)
      }

      DomainNameTooLong(len: usize) {
        description("name label data exceed 255")
        display("name label data exceed 255: {}", len)
      }

      EdnsNameNotRoot(found: ::rr::Name) {
        description("edns resource record label must be the root label (.)")
        display("edns resource record label must be the root label (.): {}", found)
      }

      LabelBytesTooLong(len: usize) {
        description("label bytes exceed 63")
        display("label bytes exceed 63: {}", len)
      }

      Message(msg: &'static str) {
        description(msg)
        display("{}", msg)
      }

      NoError {
        description("no error specified")
        display("no error specified")
      }

      NotAllBytesSent(sent: usize, expect: usize) {
        description("not all bytes were sent")
        display("not all bytes were sent: {}, expected: {}", sent, expect)
      }

      NotAllBytesReceived(received: usize, expect: usize) {
        description("not all bytes were recieved")
        display("not all bytes were recieved: {}, expected: {}", received, expect)
      }

      IncorrectMessageId(got: u16, expect: u16) {
        description("incorrectMessageId received")
        display("incorrectMessageId got: {}, expected: {}", got, expect)
      }

      IncorrectRDataLengthRead(read: usize, len: usize) {
        description("incorrect rdata length read")
        display("incorrect rdata length read: {} expected: {}", read, len)
      }

      Timeout {
        description("request timeout")
        display("request timed out")
      }

      UnknownAlgorithmTypeValue(value: u8) {
        description("algorithm type value unknown")
        display("algorithm type value unknown: {}", value)
      }

      UnknownDnsClassStr(value: String) {
        description("dns class string unknown")
        display("dns class string unknown: {}", value)
      }

      UnknownDnsClassValue(value: u16) {
        description("dns class value unknown")
        display("dns class value unknown: {}", value)
      }

      UnrecognizedLabelCode(value: u8) {
        description("unrecognized label code")
        display("unrecognized label code: {:b}", value)
      }

      UnrecognizedNsec3Flags(value: u8) {
        description("nsec3 flags should be 0b0000000*")
        display("nsec3 flags should be 0b0000000*: {:b}", value)
      }

      UnknownRecordTypeStr(value: String) {
        description("record type string unknown")
        display("record type string unknown: {}", value)
      }

      UnknownRecordTypeValue(value: u16) {
        description("record type value unknown")
        display("record type value unknown: {}", value)
      }

      // TODO: these are only necessary until TXT serialization stuff is moved
      EscapedCharOutsideCharData {
        description("escaped character outside character data")
        display("escaped character outside character data")
      }

      IllegalCharacter(ch: char) {
        description("illegal character input")
        display("illegal character input: {}", ch)
      }

      UnrecognizedChar(ch: char) {
        description("unrecognized character input")
        display("unrecognized character input: {}", ch)
      }

      BadEscapedData(string: String) {
        description("escaped data not recognized")
        display("escaped data not recognized: {}", string)
      }

      UnrecognizedOctet(octet: u32) {
        description("unrecognized octet")
        display("unrecognized octet: {:x}", octet)
      }

      UnclosedQuotedString {
        description("unclosed quoted string")
        display("unclosed quoted string")
      }

      UnclosedList {
        description("unclosed list, missing ')'")
        display("unclosed list, missing ')'")
      }

      UnrecognizedDollar(string: String) {
        description("unrecognized dollar content")
        display("unrecognized dollar content: {}", string)
      }

      EOF {
        description("unexpected end of input")
        display("unexpected end of input")
      }

      IllegalState(string: &'static str) {
        description("illegal state")
        display("illegal state: {}", string)
      }

      UnexpectedToken(token: ::serialize::txt::Token) {
        description("unrecognized token in stream")
        display("unrecognized token in stream: {:?}", token)
      }

      MissingToken(string: String) {
        description("token is missing")
        display("token is missing: {}", string)
      }

      CharToIntError(ch: char) {
        description("invalid numerical character")
        display("invalid numerical character: {}", ch)
      }

      ParseTimeError(string: String) {
        description("invalid time string")
        display("invalid time string: {}", string)
      }
    }
}

#[cfg(not(feature = "openssl"))]
pub mod not_openssl {
    use std;

    #[derive(Debug)]
    pub struct SslErrorStack;

    impl std::fmt::Display for SslErrorStack {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            Ok(())
        }
    }


    impl std::error::Error for SslErrorStack {
        fn description(&self) -> &str {
            "openssl feature not enabled"
        }
    }
}