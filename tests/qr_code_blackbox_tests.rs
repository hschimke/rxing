/*
 * Copyright 2008 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rxing::{qrcode::QRCodeReader, BarcodeFormat};

mod common;

/**
 * @author Sean Owen
 */

#[test]
fn qrcode_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-1",
        Box::new(QRCodeReader {}),
        rxing::BarcodeFormat::QR_CODE,
    );
    // super("src/test/resources/blackbox/qrcode-1", new MultiFormatReader(), BarcodeFormat.QR_CODE);
    tester.addTest(17, 17, 0.0);
    tester.addTest(14, 14, 90.0);
    tester.addTest(17, 17, 180.0);
    tester.addTest(14, 14, 270.0);

    tester.testBlackBox();
}

/**
 * @author Sean Owen
 */

#[test]
fn qrcode_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-2",
        Box::new(QRCodeReader {}),
        BarcodeFormat::QR_CODE,
    );
    tester.addTest(31, 31, 0.0);
    tester.addTest(30, 30, 90.0);
    tester.addTest(30, 30, 180.0);
    tester.addTest(30, 30, 270.0);

    tester.testBlackBox();
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[test]
fn qrcode_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-3",
        Box::new(QRCodeReader {}),
        BarcodeFormat::QR_CODE,
    );
    tester.addTest(38, 38, 0.0);
    tester.addTest(39, 39, 90.0);
    tester.addTest(36, 36, 180.0);
    tester.addTest(39, 39, 270.0);

    tester.testBlackBox();
}

/**
 * Tests of various QR Codes from t-shirts, which are notoriously not flat.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[test]
fn qrcode_black_box4_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-4",
        Box::new(QRCodeReader {}),
        BarcodeFormat::QR_CODE,
    );
    tester.addTest(36, 36, 0.0);
    tester.addTest(35, 35, 90.0);
    tester.addTest(35, 35, 180.0);
    tester.addTest(35, 35, 270.0);

    tester.testBlackBox();
}

/**
 * Some very difficult exposure conditions including self-shadowing, which happens a lot when
 * pointing down at a barcode (i.e. the phone's shadow falls across part of the image).
 * The global histogram gets about 5/15, where the local one gets 15/15.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[test]
fn qrcode_black_box5_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-5",
        Box::new(QRCodeReader {}),
        BarcodeFormat::QR_CODE,
    );
    tester.addTest(19, 19, 0.0);
    tester.addTest(19, 19, 90.0);
    tester.addTest(19, 19, 180.0);
    tester.addTest(19, 19, 270.0);

    tester.testBlackBox();
}

/**
 * These tests are supplied by Tim Gernat and test finder pattern detection at small size and under
 * rotation, which was a weak spot.
 */

#[test]
fn qrcode_black_box6_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-6",
        Box::new(QRCodeReader {}),
        BarcodeFormat::QR_CODE,
    );
    tester.addTest(15, 15, 0.0);
    tester.addTest(14, 14, 90.0);
    tester.addTest(13, 13, 180.0);
    tester.addTest(14, 14, 270.0);

    tester.testBlackBox();
}
