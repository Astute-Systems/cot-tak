//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! Create a navigation XML/protobuf object.
//! 
//! ``` xml
//! ```
//! <?xml version="1.0" encoding="utf-16"?>
//! <event version="2.0" uid="484cb239-18bc-4f47-9445-d9fc8ac2368c" type="b-m-r" how="h-e" time="2025-02-16T02:51:52.000Z" start="2025-02-16T02:51:52.000Z" stale="2025-03-18T02:51:52.000Z">
//!     <point lat="0" lon="0" hae="0" ce="9999999" le="9999999" />
//!     <detail>
//!         <contact callsign="Drive to Brisbane" />
//!         <link uid="b66e34d2-5995-4d00-b8db-fb166f0428d2" remarks="" relation="" callsign="Drive to Brisbane SP" type="b-m-p-w" point="-27.530133,152.985465" />
//!         <link uid="ab52a454-2a4f-4745-81df-c12affe0c148" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.530146,152.985785" />
//!         <link uid="43d95f25-83fa-4ea5-9289-7dd477e3275b" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.530336,152.986862" />
//!         <link uid="4a6a973c-fb3b-4aee-acfc-52ef002d72e7" remarks="" relation="" callsign="Turn Right" type="b-m-p-w" point="-27.530405,152.987500" />
//!         <link uid="05b27474-4e4f-4bd9-9810-fd93a357add3" remarks="" relation="" callsign="Turn Right" type="b-m-p-w" point="-27.531608,152.987287" />
//!         <link uid="113e0fd4-42b4-4d97-966e-72a8a0fc09a9" remarks="" relation="" callsign="Turn Left" type="b-m-p-w" point="-27.531331,152.985510" />
//!         <link uid="efaaf83d-fc2d-48ea-a2e5-f29101339603" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.531549,152.985420" />
//!         <link uid="7db21b38-4293-428c-b10d-3df35c50daba" remarks="" relation="" callsign="Turn Right" type="b-m-p-w" point="-27.532722,152.985250" />
//!         <link uid="776de6dc-9b9a-49b2-ade1-a789820eca77" remarks="" relation="" callsign="Streight Ahead" type="b-m-p-w" point="-27.532415,152.983041" />
//!         <link uid="14eee5ad-548f-47df-88c5-7925523f3c2a" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.532196,152.981177" />
//!         <link uid="b7d9c91d-b05f-496e-abf6-479529e80a08" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.532007,152.979769" />
//!         <link uid="a9ae09b1-b3ad-48c6-ad13-3b055a3a7b6b" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.531942,152.979536" />
//!         <link uid="3b055ca1-fb72-43ed-b8eb-8a63462f5ea7" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.531862,152.979074" />
//!         <link uid="f302fddc-e92e-4e34-a3a2-1804fd6fb920" remarks="" relation="" callsign="Turn Right" type="b-m-p-w" point="-27.531675,152.977787" />
//!         <link uid="1b11bf18-2471-40ca-bf25-4dcc8921b4b2" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.531070,152.977858" />
//!         <link uid="35503e22-692b-42cd-a01c-071e9a1281ba" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.530118,152.978023" />
//!         <link uid="071a0019-e438-43ad-bc94-83a71a9e9b81" remarks="" relation="" callsign="Turn Right" type="b-m-p-w" point="-27.529702,152.978103" />
//!         <link uid="b82fb0c7-777f-4652-a1c9-212ec795585d" remarks="" relation="" callsign="Turn Left" type="b-m-p-w" point="-27.529826,152.979032" />
//!         <link uid="d75bff2f-b3a0-4334-9c5c-55b351abfe4a" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.529746,152.979074" />
//!         <link uid="6bd1a07d-1f40-4a94-bfa5-a61f1294cdfd" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.528785,152.978919" />
//!         <link uid="1a8fb14e-af98-45ce-a4c2-f7a2f18b20b5" remarks="" relation="" callsign="Speed Up" type="b-m-p-w" point="-27.527818,152.978758" />
//!         <link uid="07df39a0-3a4f-49f5-908c-50acde51deee" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.527196,152.978582" />
//!         <link uid="ebf6eafd-31fd-43c1-a5bc-3bb8f3062fe1" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.525885,152.977997" />
//!         <link uid="8978c871-1ddc-4d3d-96b6-251a5cf841cf" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.525288,152.977726" />
//!         <link uid="3b0f3a0e-39fb-48ce-a393-4423c5b36e9d" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.524311,152.977268" />
//!         <link uid="561840be-cba5-40f3-b0a6-340b5b9ddbe8" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.522827,152.976578" />
//!         <link uid="88730ffd-3bae-45ab-bb32-f367e1f7d3e8" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.521250,152.975889" />
//!         <link uid="cbcd68e5-ae7b-497b-9c19-367b1697b840" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.519563,152.975109" />
//!         <link uid="3331098f-dc0a-4d15-9eea-77f804051671" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.518125,152.974454" />
//!         <link uid="4ab1664d-2a03-4bc9-a363-241a608c45e7" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.516467,152.973631" />
//!         <link uid="cb153fd6-be89-4760-802b-b066ab111667" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.515576,152.973200" />
//!         <link uid="f59edf0a-8151-4cec-a178-b31821ce836e" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.514016,152.972669" />
//!         <link uid="25847246-662f-491f-9cd9-efa9a0183129" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.513724,152.972598" />
//!         <link uid="5f96b268-d76a-4fc1-a75a-670076e09078" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.513514,152.972565" />
//!         <link uid="7c774427-0e87-4852-8fdb-7e198cca4bc6" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.513014,152.972479" />
//!         <link uid="9d610773-9a26-4549-a167-cbf16439e4dc" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.512144,152.972456" />
//!         <link uid="8b355746-63e8-4def-bd49-dc474c9780ab" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.511034,152.972494" />
//!         <link uid="395e9c58-af11-4d11-af26-cee41c6764cc" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.509970,152.972545" />
//!         <link uid="b694b36c-8900-48a2-8dd5-ef4786baad85" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.509025,152.972635" />
//!         <link uid="f92bd6b9-cf84-47d7-846b-ddc35de15418" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.508415,152.972753" />
//!         <link uid="2ded2f28-7e01-4b8a-86a4-b0a6f1bb63a0" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.507650,152.972951" />
//!         <link uid="2acd9051-76df-4723-a6b0-547b00098d7d" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.506893,152.973215" />
//!         <link uid="32cc311b-96b4-4c18-a57c-8e3200df9612" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.504699,152.973908" />
//!         <link uid="187bef29-7651-476b-a8bb-5b30501bafe0" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.504468,152.974026" />
//!         <link uid="3e5120e5-2c61-4dbd-9e2e-f285b2faa079" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.504241,152.974238" />
//!         <link uid="0b595385-eb61-4ccc-8026-55fbe25d3cfa" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.503804,152.974789" />
//!         <link uid="6993e9f9-34ed-4ea7-81f8-147bbcf57132" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.502858,152.975478" />
//!         <link uid="f755bdcc-4df0-466d-aa30-9c4fe412a922" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.502606,152.975713" />
//!         <link uid="9f4d9c63-4f5f-4665-aa01-578ca3c2b16c" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.502232,152.975855" />
//!         <link uid="ba31b86f-a189-4bee-bcde-c70cbae6db18" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.502089,152.975826" />
//!         <link uid="2325ca6c-07ad-4fef-bc71-25379bc1db78" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.501916,152.975822" />
//!         <link uid="759516b2-6f68-411e-9b34-613b1ff28f76" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.501433,152.975798" />
//!         <link uid="1e68d643-cb2a-4fab-850a-7a63bf7601cb" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.499659,152.975760" />
//!         <link uid="9161157d-9570-4a3e-acdd-2e4861ee9a03" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.498175,152.975756" />
//!         <link uid="e95e5dba-4647-4b97-869a-d33663855ab7" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.497873,152.975723" />
//!         <link uid="20dae941-762e-4545-a502-69eded6ce170" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.497440,152.975676" />
//!         <link uid="9f002492-5abf-4c3c-b0ae-548fd6c05d71" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.496952,152.975289" />
//!         <link uid="b8a31a75-283e-4e27-ac17-a6f875234dff" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.496700,152.975006" />
//!         <link uid="951d16ee-83c8-44f0-b193-04ca423b8c55" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.496498,152.974808" />
//!         <link uid="f3fbe6e0-8e6a-46a7-8cd2-c2ea325fb7a3" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.496090,152.975152" />
//!         <link uid="e8e266ce-6462-4ba5-ad8a-74f89d374623" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.495186,152.975859" />
//!         <link uid="49fea826-3ca5-492a-aded-ec944ad89492" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.494585,152.976335" />
//!         <link uid="a0da022f-1380-4e42-a896-0c235958d835" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.493724,152.976830" />
//!         <link uid="171e24f6-36e0-491c-a158-c3743e869ca8" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.493202,152.977123" />
//!         <link uid="30b1ef51-9ba9-47b8-bf39-fb9ed90a4b6b" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.492891,152.977363" />
//!         <link uid="2e4ab6b5-402c-4088-8af9-a555fc6ab0d0" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.492311,152.978146" />
//!         <link uid="ab51eee7-bb8f-4920-9f15-a03992e38c67" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.492034,152.978556" />
//!         <link uid="846dfbbe-91ea-42ba-a86a-f19b13ca8cd0" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.491874,152.979013" />
//!         <link uid="030fa733-356a-4ff2-acae-40369513298f" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.491819,152.979564" />
//!         <link uid="51c59aaa-4053-4a04-8cf7-dae4133f7e7d" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.491735,152.981205" />
//!         <link uid="501205fa-e7be-4e34-84d8-1650e05349eb" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.491664,152.981883" />
//!         <link uid="77c0d962-f25b-4826-9ad8-d28f13d05826" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.491416,152.983123" />
//!         <link uid="df4341ec-bde3-4980-bd37-32f2bd3f8c42" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.491235,152.983599" />
//!         <link uid="2ebaff1b-0510-4ca2-991a-3c4792463bca" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.490899,152.984693" />
//!         <link uid="6f34d84f-694d-4528-a3c3-364a15e7adfb" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.490756,152.985202" />
//!         <link uid="4803bf22-770b-46f1-9d20-4133a0ab7200" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.490609,152.985546" />
//!         <link uid="eccd28a2-ae73-4128-a54c-ccefde49de0b" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.489793,152.987106" />
//!         <link uid="e7162771-306c-42c0-aeb4-32bcd65bdca4" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.489259,152.988096" />
//!         <link uid="00fc7758-3b15-4ffc-aac7-1d90c061cc2a" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.488738,152.989025" />
//!         <link uid="23021a2d-ee37-4945-9335-792ad3d93012" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.488276,152.989831" />
//!         <link uid="aa2264dd-dcd7-4210-b42e-0cd8ca65b549" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.488192,152.989934" />
//!         <link uid="ea3192d7-7763-4047-84bb-911a40b0ab93" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.487969,152.990146" />
//!         <link uid="5126992c-731b-4ae3-bf57-2a063658560d" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.487624,152.990467" />
//!         <link uid="ccc96308-09d9-4b63-a32c-5c638620d383" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.486788,152.991197" />
//!         <link uid="31586d31-c41d-4cd1-85d4-785ad51a860f" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.486317,152.991758" />
//!         <link uid="d78862b8-1912-4168-a340-c8bf70b1e8a4" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.486094,152.992074" />
//!         <link uid="d7465fc7-871b-4ab6-881d-6ef7a2e572b0" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.486027,152.992333" />
//!         <link uid="1c511fa7-87a4-473a-991e-7457755afc7b" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.486010,152.992583" />
//!         <link uid="feb30e1a-d4db-4626-8e5a-63f1564a0138" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.485981,152.993055" />
//!         <link uid="e37d19f4-561a-4c1c-a7c4-11754547ea9c" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.485766,152.993224" />
//!         <link uid="6104c6e7-fec2-4867-87e1-9d91347321de" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.485338,152.993375" />
//!         <link uid="7e337ffd-2825-439b-8b90-dd438acd1286" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.484934,152.993516" />
//!         <link uid="72a44090-82f6-47f4-b442-75594c38173f" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.484778,152.993696" />
//!         <link uid="faef2b47-8e8a-4cec-9e49-b1e7608312dd" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.484442,152.994195" />
//!         <link uid="202d1ad8-8630-44f4-972d-0059ffd78704" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.483774,152.995048" />
//!         <link uid="629643de-9ac7-402e-beb1-98121a5bf812" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.483660,152.995326" />
//!         <link uid="38416bc7-870a-4d1f-8f3a-8bc4140d2b48" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.483568,152.995571" />
//!         <link uid="f33ff26d-8cb6-4d2d-80a7-a68ca1a2c64f" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.483379,152.995911" />
//!         <link uid="f957f84d-11a8-4308-b8cb-9016e725f19e" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.482803,152.996359" />
//!         <link uid="f1eb2d51-dc61-4b66-ac0b-2fb412c7c5b1" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.482412,152.996571" />
//!         <link uid="c4a12c72-6bd5-4658-a3eb-df8303fcb810" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.482240,152.996066" />
//!         <link uid="b5632f08-90f0-4fa6-ad1a-17e6eb4e9a8e" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.482084,152.995524" />
//!         <link uid="0c31033e-3d7f-4fb1-a97b-7775bb6424fd" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.481807,152.994685" />
//!         <link uid="c3c78617-d843-41a6-a8ed-965e594271b6" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.480819,152.991443" />
//!         <link uid="1059faeb-45ff-41d8-a0bc-4b6231b3c20e" remarks="" relation="" callsign="" type="b-m-p-c" point="-27.480651,152.991174" />
//!         <link uid="eb4f0a62-5366-4a99-88b2-cb8d04328a0f" remarks="" relation="" callsign="End" type="b-m-p-w" point="-27.480318,152.990896" />
//!         <link_attr color="-16711936" method="Driving" direction="Infil" routetype="Primary" order="Ascending Check Points" />
//!         <remarks />
//!         <archive />
//!         <__routeinfo>
//!             <__navcues>
//!                 <__navcue id="b66e34d2-5995-4d00-b8db-fb166f0428d2" text="Stay Straight" voice="Stay Straight" />
//!                 <__navcue id="4a6a973c-fb3b-4aee-acfc-52ef002d72e7" text="Turn Right" voice="Turn Right" />
//!                 <__navcue id="05b27474-4e4f-4bd9-9810-fd93a357add3" text="Turn Right" voice="Turn Right" />
//!                 <__navcue id="113e0fd4-42b4-4d97-966e-72a8a0fc09a9" text="Turn Left" voice="Turn Left" />
//!                 <__navcue id="7db21b38-4293-428c-b10d-3df35c50daba" text="Turn Right" voice="Turn Right" />
//!                 <__navcue id="776de6dc-9b9a-49b2-ade1-a789820eca77" text="Stay Straight" voice="Stay Straight" />
//!                 <__navcue id="f302fddc-e92e-4e34-a3a2-1804fd6fb920" text="Turn Right" voice="Turn Right" />
//!                 <__navcue id="071a0019-e438-43ad-bc94-83a71a9e9b81" text="Turn Right" voice="Turn Right" />
//!                 <__navcue id="b82fb0c7-777f-4652-a1c9-212ec795585d" text="Turn Left" voice="Turn Left" />
//!                 <__navcue id="eb4f0a62-5366-4a99-88b2-cb8d04328a0f" text="Stop" voice="Stop" />
//!                 <__navcue id="1a8fb14e-af98-45ce-a4c2-f7a2f18b20b5" text="Speed Up" voice="Speed Up" />
//!             </__navcues>
//!         </__routeinfo>
//!     </detail>
//! </event>
//! ```
//! 

