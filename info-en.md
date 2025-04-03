# Packing Assistant

## Description

Cross-border e-commerce businesses ship products overseas, with each product packaged in a regular rectangular box called a "standard item". Each standard item has attributes of "ID", "destination", "length", "width", "height", and "weight". The units for length, width, and height are "centimeters", and the unit for weight is "kilograms". Standard items of the same type have identical length, width, height, and weight.

Due to the high quantity, light weight, and small size of individual standard items in cross-border e-commerce shipments, logistics companies require e-commerce businesses to use N (N≥1) outer cardboard boxes that meet size and weight constraints to package goods for convenient transportation. Logistics companies charge based on the volume of the outer boxes, and the e-commerce businesses must order and purchase these boxes themselves. Therefore, the cost of outer boxes and box size become two key determining factors in logistics costs for cross-border e-commerce businesses.

You are now asked to develop a user-friendly, standalone packing assistant system that does not use any database management system to help cross-border e-commerce businesses save on logistics costs. The shipping destinations supported by logistics companies and their constraints are shown in Table 1, and the parameters for outer boxes are shown in Table 2.

## Table 1: Logistics Company Supported Shipping Destinations and Constraints

| Destination | Constraints | Product Dimensions (Example) |
|---|---|---|
| Australia | 1. No side of the outer box should exceed 63 cm;2. The total weight of a single outer box should not exceed 22kg. | Standard item (AUN11-ZY0009)L×W×H: 30cm×20cm×8cmWeight: 0.68 kg |
| USA | 1. No side of the outer box should exceed 63 cm;2. The total weight of a single outer box should not exceed 22 kg. | Standard item (USY6-05-0314)L×W×H: 49cm×21cm×8cmWeight: 6.999 kg |
| UK | 1. No side of the outer box should exceed 63 cm;2. The total weight of a single outer box should not exceed 15kg. | Standard item (UKB8-ZY0009)L×W×H: 30cm×20cm×8cmWeight: 0.68 kg |
| Germany | 1. No side of the outer box should exceed 63cm;2. The total weight of a single outer box should not exceed 22.5 kg. | Standard item (DEW1-ZB0004)L×W×H: 45cm×29cm×17cmWeight: 1.5 kg |
| Japan | 1. The length of the outer box should not exceed 60cm, width and height should not exceed 50cm;2. The total weight of a single outer box should not exceed 40kg. | Standard item (JPY5-GR049)L×W×H: 25cm×25cm×10cmWeight: 0.82 kg |

## Table 2: Outer Box Parameters

| No. | Name | Unit | Remarks |
|---|---|---|---|
| 1 | Outer box unit weight | kg/m² (kilograms per square meter) | Value: 0.54 |
| 2 | Outer box cardboard thickness | cm (centimeters) | Value: 0.6 |
