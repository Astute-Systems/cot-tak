//
// Copyright (c) 2025, Astute Systems PTY LTD
//
// This file is part of the VivoeX SDK project developed by Astute Systems.
//
// See the commercial LICENSE file in the project root for full license details.
//
//! This module contains the types used in the Cursor on Target Data (CoT) messages.

use strum_macros::EnumIter;

#[derive(PartialEq, EnumIter, Copy, Clone)]

/// Cursor on Target (CoT) Type enumeration
pub enum CoTType {
    /// Air Track
    AirAirtrack,
    /// CIVIL AIRCRAFT
    AirCiv,
    /// FIXED WING
    AirCivFixed,
    /// FIXED WING RPV/Drone
    AirCivFixedRpvDroneUav,
    /// ROTARY WING
    AirCivRotary,
    /// LIGHTER THAN AIR
    AirCivBlimp,
    /// FIXED WING
    AirMilFixed,
    /// FIXED WING GUNSHIP
    AirMilFixedGunship,
    /// ATTACK/STRIKE
    AirMilFixedAttackStrike,
    /// BOMBER
    AirMilFixedBomber,
    /// CARGO AIRLIFT (TRANSPORT)
    AirMilFixedTransport,
    /// CARGO AIRLIFT (HEAVY)
    AirMilFixedTransportHeavy,
    /// CARGO AIRLIFT (LIGHT)
    AirMilFixedTransportLight,
    /// CARGO AIRLIFT (MEDIUM)
    AirMilFixedTransportMedium,
    /// AIRBORNE COMMAND POST (C2)
    AirMilFixedC2,
    /// FIGHTER
    AirMilFixedFighter,
    /// INTERCEPTOR
    AirMilFixedInterceptor,
    /// COMBAT SEARCH AND RESCUE (CSAR)
    AirMilFixedCsar,
    /// ELECTRONIC COUNTERMEASURES (ECM/JAMMER)
    AirMilFixedEcmJammer,
    /// TANKER
    AirMilFixedTanker,
    /// VSTOL
    AirMilFixedVstol,
    /// SPECIAL OPERATIONS FORCES (SOF)
    AirMilFixedSof,
    /// MEDEVAC
    AirMilFixedMedevac,
    /// PATROL
    AirMilFixedPatrol,
    /// MINE COUNTERMEASURES
    AirMilFixedMinecountermeasures,
    /// ANTISURFACE WARFARE/ASUW
    AirMilFixedAsuw,
    /// DRONE (RPV/UAV)
    AirMilFixedDroneRpvUav,
    /// RECONNAISSANCE
    AirMilFixedReconn,
    /// AIRBORNE EARLY WARNING (AEW)
    AirMilFixedAew,
    /// PHOTOGRAPHIC
    AirMilFixedPhoto,
    /// ELECTRONIC SURVEILLANCE MEASURES
    AirMilFixedEsm,
    /// ANTISUBMARINE WARFARE (ASW) CARRIER BASED
    AirMilFixedAswCarrierbased,
    /// TRAINER
    AirMilFixedTrainer,
    /// UTILITY
    AirMilFixedUtility,
    /// UTILITY (HEAVY)
    AirMilFixedUtilityHeavy,
    /// UTILITY (LIGHT)
    AirMilFixedUtilityLight,
    /// UTILITY (MEDIUM)
    AirMilFixedUtilityMedium,
    /// COMMUNICATIONS (C3I)
    AirMilFixedC3I,
    /// ROTARY WING
    AirMilRotor,
    /// ATTACK
    AirMilRotorAttack,
    /// CARGO AIRLIFT (TRANSPORT)
    AirMilRotorTransport,
    /// CARGO AIRLIFT (HEAVY)
    AirMilRotorTransportHeavy,
    /// CARGO AIRLIFT (LIGHT)
    AirMilRotorTransportLight,
    /// CARGO AIRLIFT (MEDIUM)
    AirMilRotorTransportMedium,
    /// AIRBORNE COMMAND POST (C2)
    AirMilRotorC2,
    /// COMBAT SEARCH AND RESCUE (CSAR)
    AirMilRotorCsar,
    /// MINE COUNTERMEASURES
    AirMilRotorMinecountermeasures,
    /// ELECTRONIC COUNTERMEASURES (ECM/JAMMER)
    AirMilRotorEcmJammer,
    /// TANKER
    AirMilRotorTanker,
    /// SPECIAL OPERATIONS FORCES (SOF)
    AirMilRotorSof,
    /// MEDEVAC
    AirMilRotorMedevac,
    /// DRONE (RPV/UAV)
    AirMilRotorDroneRpvUav,
    /// RECONNAISSANCE
    AirMilRotorReconnaissance,
    /// ANTISUBMARINE WARFARE/MPA
    AirMilRotorAswMpa,
    /// TRAINER
    AirMilRotorTrainer,
    /// UTILITY
    AirMilRotorUtility,
    /// UTILITY (HEAVY)
    AirMilRotorUtilityHeavy,
    /// UTILITY (LIGHT)
    AirMilRotorUtilityLight,
    /// UTILITY (MEDIUM)
    AirMilRotorUtilityMedium,
    /// LIGHTER THAN AIR
    AirMilBlimp,
    /// WEAPON
    AirWeapon,
    /// DECOY
    AirWeaponDecoy,
    /// MISSILE IN FLIGHT
    AirWeaponMissileinflight,
    /// AIR LAUNCHED MISSILE
    AirWeaponMissileAirlaunched,
    /// AIR TO AIR MISSILE (AAM)
    AirWeaponMissileAirtoair,
    /// AIR TO SURFACE MISSILE (ASM)
    AirWeaponMissileAirtosurface,
    /// LAND ATTACK MISSILE
    AirWeaponMissileAttackmissile,
    /// SURFACE/LAND LAUNCHED MISSILE
    AirWeaponMissileLandlaunchedmissile,
    /// SURFACE TO AIR MISSILE (SAM)
    AirWeaponMissileSam,
    /// SAM (FixedSite)
    AirWeaponMissileSamFixedsite,
    /// SAM (Manpad)
    AirWeaponMissileSamManpad,
    /// SAM (Mobile)
    AirWeaponMissileSamMobile,
    /// SURFACE TO SURFACE MISSILE (SSM)
    AirWeaponMissileSurfacetosurfaceSsm,
    /// SUBSURFACE TO SURFACE MISSILE (S/SSM)
    AirWeaponMissileSubsurfacetosurfaceSSsm,
    /// GROUND TRACK
    Ground,
    /// EQUIPMENT
    GndEquipment,
    /// SENSOR
    GndEquipSensor,
    /// EMPLACED SENSOR
    GndEquipSensorEmplaced,
    /// RADAR
    GndEquipSensorRadar,
    /// GROUND VEHICLE
    GndEquipVehicle,
    /// Armor (GUN)
    GndEquipVehicArmorGun,
    /// Armor (APC)
    GndEquipVehicArmorApc,
    /// ARMORED PERSONNEL CARRIER RECOVERY
    GndEquipVehicArmorApcRecovery,
    /// C2V/ACV
    GndEquipVehicArmorC2VAcv,
    /// ARMORED INFANTRY
    GndEquipVehicArmorInfantry,
    /// LIGHT ARMORED VEHICLE
    GndEquipVehicArmorLight,
    /// COMBAT SERVICE SUPPORT VEHICLE
    GndEquipVehicArmorCombatservicesupport,
    /// TANK
    GndEquipVehicArmorTank,
    /// TANK HEAVY
    GndEquipVehicArmorTankHeavy,
    /// TANK HEAVY RECOVERY
    GndEquipVehicArmorTankHeavyRecovery,
    /// TANK LIGHT
    GndEquipVehicArmorTankLight,
    /// TANK LIGHT RECOVERY
    GndEquipVehicArmorTankLightRecovery,
    /// TANK MEDIUM
    GndEquipVehicArmorTankMedium,
    /// TANK MEDIUM RECOVERY
    GndEquipVehicArmorTankMediumRecovery,
    /// CIVILIAN VEHICLE
    GndEquipVehicCivilian,
    /// ENGINEER VEHICLE
    GndEquipVehicEngineer,
    /// MINE CLEARING VEHICLE
    GndEquipVehicMineclearingvehicle,
    /// ARMORED MOUNTED MINE CLEARING VEHICLE
    GndEquipVehicArmoredmountedmineclearingvehicle,
    /// TRAILER MOUNTED MINE CLEARING VEHICLE
    GndEquipVehicTrailermountedmineclearingvehicle,
    /// BRIDGE
    GndEquipVehicBridge,
    /// CONSTRUCTION VEHICLE
    GndEquipVehicConstruction,
    /// DOZER
    GndEquipVehicDozer,
    /// EARTHMOVER
    GndEquipVehicEarthmover,
    /// MINE LAYING VEHICLE
    GndEquipVehicMinelayingvehicle,
    /// TRUCK MOUNTED WITH VOLCANO
    GndEquipVehicTruckmountedwithvolcano,
    /// ARMORED CARRIER WITH VOLCANO
    GndEquipVehicArmoredcarrierwithvolcano,
    /// Locomotive
    GndEquipVehicLocomotive,
    /// UTILITY VEHICLE
    GndEquipVehicUtility,
    /// BUS
    GndEquipVehicBus,
    /// LIMITED CROSS COUNTRY TRUCK
    GndEquipVehicLimitedcrosscountrytruck,
    /// WATER CRAFT
    GndEquipVehicBoat,
    /// Boat (Large)
    GndEquipVehicBoatLarge,
    /// Boat (Med)
    GndEquipVehicBoatMed,
    /// Boat (Small)
    GndEquipVehicBoatSmall,
    /// SEMI
    GndEquipVehicSemi,
    /// CROSS COUNTRY TRUCK
    GndEquipVehicCrosscountrytruck,
    /// Ambulance
    GndEquipVehicAmbulance,
    /// WEAPON
    GndEquipWeapon,
    /// AIR DEFENSE GUN
    GndEquipWeaponAirdefensegun,
    /// AAA (Heavy)
    GndEquipWeaponAaaHeavy,
    /// AAA (Light)
    GndEquipWeaponAaaLight,
    /// AAA (Medium)
    GndEquipWeaponAaaMedium,
    /// DIRECT FIRE GUN
    GndEquipWeaponDirectfire,
    /// DIRECT FIRE GUN HEAVY
    GndEquipWeaponDirectfireHeavy,
    /// DIRECT FIRE GUN HEAVY SELF PROPELLED
    GndEquipWeaponDirectfireHeavySelfpropelled,
    /// DIRECT FIRE GUN LIGHT
    GndEquipWeaponDirectfireLight,
    /// DIRECT FIRE GUN LIGHT SELF PROPELLED
    GndEquipWeaponDirectfireLightSelfpropelled,
    /// DIRECT FIRE GUN MEDIUM
    GndEquipWeaponDirectfireMedium,
    /// DIRECT FIRE GUN MEDIUM SELF PROPELLED
    GndEquipWeaponDirectfireMediumSelfpropelled,
    /// ANTI TANK GUN
    GndEquipWeaponAnittankgun,
    /// ANTI TANK GUN HEAVY
    GndEquipWeaponAtgHeavy,
    /// ANTI TANK GUN LIGHT
    GndEquipWeaponAtgLight,
    /// ANTI TANK GUN MEDIUM
    GndEquipWeaponAtgMedium,
    /// ANTI TANK GUN RECOILLESS
    GndEquipWeaponAtgRecoilless,
    /// HOWITZER
    GndEquipWeaponHowitzer,
    /// HOWITZER HEAVY
    GndEquipWeaponHowitzerHeavy,
    /// HOWITZER HEAVY SELF PROPELLED
    GndEquipWeaponHowitzerHeavySelfpropelled,
    /// HOWITZER LIGHT
    GndEquipWeaponHowitzerLight,
    /// HOWITZER LIGHT SELF PROPELLED
    GndEquipWeaponHowitzerLightSelfpropelled,
    /// HOWITZER MEDIUM
    GndEquipWeaponHowitzerMedium,
    /// HOWITZER MEDIUM SELF PROPELLED
    GndEquipWeaponHowitzerMediumSelfpropelled,
    /// MISSILE LAUNCHER
    GndEquipWeaponMissilelauncher,
    /// AIR DEFENSE (AD) MISSILE LAUNCHER
    GndEquipWeaponMissilelauncherAdAirdefense,
    /// INTERMEDIATE RANGE AD MISSILE LAUNCHER
    GndEquipWeaponMissilelauncherAdIntermediaterange,
    /// LONG RANGE AD MISSILE LAUNCHER
    GndEquipWeaponMissilelauncherAdLongrange,
    /// SHORT RANGE AD MISSILE LAUNCHER
    GndEquipWeaponMissilelauncherAdShortrange,
    /// AD MISSILE LAUNCHER THEATER
    GndEquipWeaponMissilelauncherAdTheater,
    /// SURF SURF (SS) MISSILE LAUNCHER
    GndEquipWeaponMissilelauncherSsSurfacesurfSs,
    /// INTERMEDIATE RANGE SS MISSILE LAUNCHER
    GndEquipWeaponMissilelauncherSsIntermediaterange,
    /// LONG RANGE SS MISSILE LAUNCHER
    GndEquipWeaponMissilelauncherSsLongrange,
    /// SHORT RANGE SS MISSILE LAUNCHER
    GndEquipWeaponMissilelauncherSsShortrange,
    /// MISSILE LAUNCHER ANTITANK (AT)
    GndEquipWeaponMissilelauncherAtAntitank,
    /// MISSILE LAUNCHER AT HEAVY
    GndEquipWeaponMissilelauncherAtHeavy,
    /// MISSILE LAUNCHER AT LIGHT
    GndEquipWeaponMissilelauncherAtLight,
    /// MISSILE LAUNCHER AT MEDIUM
    GndEquipWeaponMissilelauncherAtMedium,
    /// MORTAR
    GndEquipWeaponMortar,
    /// MORTAR HEAVY
    GndEquipWeaponMortarHeavy,
    /// MORTAR LIGHT
    GndEquipWeaponMortarLight,
    /// MORTAR MEDIUM
    GndEquipWeaponMortarMedium,
    /// RIFLE/AUTOMATIC WEAPON
    GndEquipWeaponRifleAutomaticweapon,
    /// HEAVY MACHINE GUN
    GndEquipWeaponRifleHeavymachinegun,
    /// LIGHT MACHINE GUN
    GndEquipWeaponRifleLightmachinegun,
    /// RIFLE
    GndEquipWeaponRifleRifle,
    /// SINGLE ROCKET LAUNCHER
    GndEquipWeaponRocketlauncherSingle,
    /// SINGLE ROCKET LAUNCHER HEAVY
    GndEquipWeaponRocketlauncherSingleHeavy,
    /// SINGLE ROCKET LAUNCHER LIGHT
    GndEquipWeaponRocketlauncherSingleLight,
    /// SINGLE ROCKET LAUNCHER MEDIUM
    GndEquipWeaponRocketlauncherSingleMedium,
    /// ANTI TANK ROCKET LAUNCHER
    GndEquipWeaponRocketlauncherAntitank,
    /// ANTI TANK ROCKET LAUNCHER HEAVY
    GndEquipWeaponRocketlauncherAtHeavy,
    /// ANTI TANK ROCKET LAUNCHER LIGHT
    GndEquipWeaponRocketlauncherAtLight,
    /// ANTI TANK ROCKET LAUNCHER MEDIUM
    GndEquipWeaponRocketlauncherAtMedium,
    /// MULTIPLE ROCKET LAUNCHER
    GndEquipWeaponRocketlauncherMultiple,
    /// MULTIPLE ROCKET LAUNCHER HEAVY
    GndEquipWeaponRocketlauncherMultipleHeavy,
    /// MULTIPLE ROCKET LAUNCHER LIGHT
    GndEquipWeaponRocketlauncherMultipleLight,
    /// MULTIPLE ROCKET LAUNCHER MEDIUM
    GndEquipWeaponRocketlauncherMultipleMedium,
    /// GRENADE LAUNCHER
    GndEquipWeaponGrenadelauncher,
    /// GRENADE LAUNCHER HEAVY
    GndEquipWeaponGrenadelauncherHeavy,
    /// GRENADE LAUNCHER LIGHT
    GndEquipWeaponGrenadelauncherLight,
    /// GRENADE LAUNCHER MEDIUM
    GndEquipWeaponGrenadelauncherMedium,
    /// SPECIAL EQUIPMENT
    GndEquipSpecialequipment,
    /// FLAME THROWER
    GndEquipFlamethrower,
    /// LASER
    GndEquipLaser,
    /// LAND MINES
    GndEquipLandmines,
    /// CLAYMORE
    GndEquipClaymore,
    /// LESS THAN LETHAL
    GndEquipLessthanlethal,
    /// NBC EQUIPMENT
    GndEquipNbcequipment,
    /// Building
    GndBuilding,
    /// MILITARY BASE/FACILITY
    GndStructureBaseMilitary,
    /// AIRPORT/AIRBASE
    GndStructureBaseAirbase,
    /// SEAPORT/NAVAL BASE
    GndStructureBaseNavalbase,
    /// Civilian
    GndStructureCivilian,
    /// Bridge
    GndStructureCivilianBridge,
    /// Fence/Wall/Barrier
    GndStructureCivilianFenceorwallorbarrier,
    /// Canal
    GndStructureCivilianCanal,
    /// Canal Lock
    GndStructureCivilianCanalLock,
    /// Farm
    GndStructureCivilianFarm,
    /// Levy
    GndStructureCivilianLevee,
    /// Monument
    GndStructureCivilianMonument,
    /// Offices
    GndStructureCivilianOffices,
    /// Residence/Apartment/Hotel
    GndStructureCivilianResidenceorapartmentorhotel,
    /// Religous
    GndStructureCivilianReligous,
    /// Resevoir
    GndStructureCivilianResevoir,
    /// Retail
    GndStructureCivilianRetail,
    /// School
    GndStructureCivilianSchool,
    /// VIP
    GndStructureCivilianVip,
    /// Warehouse
    GndStructureCivilianWarehouse,
    /// EQUIPMENT MANUFACTURE
    GndStructureFactory,
    /// Heavy
    GndStructureFactoryHeavy,
    /// Light
    GndStructureFactoryLight,
    /// Plant Other
    GndStructureFactoryPlantother,
    /// GOVERNMENT LEADERSHIP
    GndStructureGovernment,
    /// IM Facilities
    GndStructureImfacilities,
    /// Emergency Management
    GndStructureImfacilitiesEmergencymanagement,
    /// Emergency Operations Center
    GndStructureImfacilitiesEmergencymanagementEmergencyoperationscenter,
    /// Incident Commander
    GndStructureImfacilitiesEmergencymanagementIncidentcommander,
    /// Incident Communications Center
    GndStructureImfacilitiesEmergencymanagementIncidentcommunicationscenter,
    /// Incident Command Post
    GndStructureImfacilitiesEmergencymanagementIncidentcommandpost,
    /// War Room
    GndStructureImfacilitiesEmergencymanagementWarroom,
    /// Fire/Hazmat
    GndStructureImfacilitiesFireandhazmat,
    /// Fire Department
    GndStructureImfacilitiesFireandhazmatFiredepartment,
    /// Health and Medical
    GndStructureImfacilitiesHealthandmedical,
    /// Medical Dispatch
    GndStructureImfacilitiesHealthandmedicalMedicaldispatch,
    /// Law Enforcement
    GndStructureImfacilitiesLawenforcement,
    /// Police Checkpoint
    GndStructureImfacilitiesLawenforcementPolicecheckpoint,
    /// Highway Patrol
    GndStructureImfacilitiesLawenforcementHighwaypatrol,
    /// Local Police
    GndStructureImfacilitiesLawenforcementLocalpolice,
    /// No Trespassing Zone
    GndStructureImfacilitiesLawenforcementNotrespassingzone,
    /// Sheriffs Department
    GndStructureImfacilitiesLawenforcementSheriffsdepartment,
    /// State Police
    GndStructureImfacilitiesLawenforcementStatepolice,
    /// Emergency Medical Services
    GndStructureImfacilitiesEmergencymedicalservices,
    /// Other
    GndStructureImfacilitiesOther,
    /// Agency Dispatch
    GndStructureImfacilitiesOtherAgencydispatch,
    /// EMS
    GndStructureImfacilitiesOtherAgencydispatchEms,
    /// Fire
    GndStructureImfacilitiesOtherAgencydispatchFire,
    /// Highway
    GndStructureImfacilitiesOtherAgencydispatchHighway,
    /// Media
    GndStructureImfacilitiesOtherAgencydispatchMedia,
    /// Police Local
    GndStructureImfacilitiesOtherAgencydispatchPolicelocal,
    /// Police Station
    GndStructureImfacilitiesOtherAgencydispatchPolicestation,
    /// Tow
    GndStructureImfacilitiesOtherAgencydispatchTow,
    /// Transit
    GndStructureImfacilitiesOtherAgencydispatchTransit,
    /// Border Control Checkpoint
    GndStructureImfacilitiesOtherBordercontrolcheckpoint,
    /// DMZ
    GndStructureImfacilitiesOtherDmz,
    /// Independent Service Provider
    GndStructureImfacilitiesOtherIndependentserviceprovider,
    /// Mobile Center
    GndStructureImfacilitiesOtherMobilecenter,
    /// Fire
    GndStructureImfacilitiesOtherMobilecenterFire,
    /// Medical
    GndStructureImfacilitiesOtherMobilecenterMedical,
    /// Law Enforcement
    GndStructureImfacilitiesOtherMobilecenterLawenforcement,
    /// Message Center
    GndStructureImfacilitiesOtherMessagecenter,
    /// Mobilization Center
    GndStructureImfacilitiesOtherMobilizationcenter,
    /// EMS
    GndStructureImfacilitiesOtherMobilizationcenterEms,
    /// Transit
    GndStructureImfacilitiesOtherMobilizationcenterTransit,
    /// Fire
    GndStructureImfacilitiesOtherMobilizationcenterFire,
    /// Highway
    GndStructureImfacilitiesOtherMobilizationcenterHighway,
    /// Media
    GndStructureImfacilitiesOtherMobilizationcenterMedia,
    /// Ploce Local
    GndStructureImfacilitiesOtherMobilizationcenterPolicelocal,
    /// Police Station
    GndStructureImfacilitiesOtherMobilizationcenterPolicestation,
    /// Tow
    GndStructureImfacilitiesOtherMobilizationcenterTow,
    /// Public Safety
    GndStructureImfacilitiesOtherPublicsafety,
    /// Reporting Locations
    GndStructureImfacilitiesOtherReportinglocations,
    /// Staging Area
    GndStructureImfacilitiesOtherStagingarea,
    /// EMS
    GndStructureImfacilitiesOtherStagingareaEms,
    /// Fire
    GndStructureImfacilitiesOtherStagingareaFire,
    /// Highway
    GndStructureImfacilitiesOtherStagingareaHighway,
    /// Media
    GndStructureImfacilitiesOtherStagingareaMedia,
    /// Police Local
    GndStructureImfacilitiesOtherStagingareaPolicelocal,
    /// Police Station
    GndStructureImfacilitiesOtherStagingareaPolicestation,
    /// Tow
    GndStructureImfacilitiesOtherStagingareaTow,
    /// Transit
    GndStructureImfacilitiesOtherStagingareaTransit,
    /// Security Checkpoint
    GndStructureImfacilitiesOtherSecuritycheckpoint,
    /// Segment
    GndStructureImfacilitiesOtherSegment,
    /// Transportation
    GndStructureImfacilitiesOtherTransportation,
    /// Transit Incident Management Center
    GndStructureImfacilitiesOtherTransportationTransitincidentmanagementcenter,
    /// Transit Management Center
    GndStructureImfacilitiesOtherTransportationTransitmanagementcenter,
    /// Traffic Inc Service Patrol Dispatch
    GndStructureImfacilitiesOtherTransportationTrafficincservicepatroldispatch,
    /// Traffic Management Center
    GndStructureImfacilitiesOtherTransportationTrafficmanagementcenter,
    /// Temporary
    GndStructureImfacilitiesOtherTemp,
    /// Fire
    GndStructureImfacilitiesOtherTempFire,
    /// Medical
    GndStructureImfacilitiesOtherTempMedical,
    /// Law Enforcement
    GndStructureImfacilitiesOtherTempLawenforcement,
    /// Public Works
    GndStructureImfacilitiesPublicworks,
    /// MILITARY MATERIEL FACILITY
    GndStructureMilMilitarymaterielfacility,
    /// AIRCRAFT PRODUCTION/ASSEMBLY
    GndStructureMilAircraftproduction,
    /// CHEMICAL/BIOLOGICAL WARFARE PRODUCTION
    GndStructureMilNbcwarfareproduction,
    /// AMMUNITION AND EXPLOSIVES PRODUCTION
    GndStructureMilAmmunitionproduction,
    /// NUCLEAR ENERGY
    GndStructureMilNuclearenergy,
    /// ATOMIC ENERGY REACTOR
    GndStructureMilAtomicenergyreactor,
    /// NUCLEAR MATERIAL PRODUCTION
    GndStructureMilNuclearmaterialproduction,
    /// WEAPONS GRADE
    GndStructureMilNuclearWeaponsgrade,
    /// NUCLEAR MATERIAL STORAGE
    GndStructureMilNuclearmaterialstorage,
    /// ARMAMENT PRODUCTION
    GndStructureMilArmamentproduction,
    /// MISSILE/SPACE SYSTEM PRODUCTION
    GndStructureMilMissileSpacesystemproduction,
    /// ENGINEERING EQUIPMENT PRODUCTION
    GndStructureMilEngineeringequipmentproduction,
    /// BRIDGE
    GndStructureMilBridge,
    /// Bridge (Large)
    GndStructureMilBridgeLarge,
    /// Bridge (Med)
    GndStructureMilBridgeMed,
    /// Bridge (Small)
    GndStructureMilBridgeSmall,
    /// Canal
    GndStructureMilCanal,
    /// SHIP CONSTRUCTION
    GndStructureMilShipconstruction,
    /// MILITARY VEHICLE PRODUCTION
    GndStructureMilMilitaryvehicleproduction,
    /// PROCESSING FACILITY
    GndStructureProcessingfacility,
    /// DECON
    GndStructureDecon,
    /// RAW MATERIAL PRODUCTION/STORAGE
    GndStructureRawmaterialproductionStorage,
    /// MINE
    GndStructureMine,
    /// NBC
    GndStructureNbc,
    /// BIOLOGICAL
    GndStructureBiological,
    /// CHEMICAL
    GndStructureChemical,
    /// NUCLEAR
    GndStructureNuclear,
    /// PETROLEUM/GAS/OIL
    GndStructurePetroleumGasOil,
    /// Refinery
    GndStructurePetroleumGasOilRefinery,
    /// TRANSPORT FACILITY
    GndStructureTransportfacility,
    /// Airport
    GndStructureTransportfacilityAirport,
    /// Helibase
    GndStructureTransportfacilityHelibase,
    /// Helispot
    GndStructureTransportfacilityHelispot,
    /// Land Port
    GndStructureTransportfacilityLandport,
    /// Parking Garage
    GndStructureTransportfacilityParkinggarage,
    /// Railroad
    GndStructureTransportfacilityRailroad,
    /// Seaport
    GndStructureTransportfacilitySeaport,
    /// SERVICE, RESEARCH, UTILITY FACILITY
    GndStructureUtilityServiceResearchUtilityfacility,
    /// ELECTRIC POWER FACILITY
    GndStructureUtilityElectricpower,
    /// Coal Power Plant
    GndStructureUtilityElectricpowerCoalpowerplant,
    /// Hydroelectric Power Plant
    GndStructureUtilityElectricpowerHydroelectricpowerplant,
    /// Other Power
    GndStructureUtilityElectricpowerOtherpower,
    /// Power Substation
    GndStructureUtilityElectricpowerPowersubstation,
    /// DAM
    GndStructureUtilityDam,
    /// FOSSIL FUEL
    GndStructureUtilityFossilfuel,
    /// NUCLEAR PLANT
    GndStructureUtilityNuclearplant,
    /// PUBLIC WATER SERVICES
    GndStructureUtilityPublicwaterservices,
    /// TECHNOLOGICAL RESEARCH FACILITY
    GndStructureUtilityTechnologicalresearch,
    /// TELECOMMUNICATIONS FACILITY
    GndStructureUtilityTelecommunications,
    /// Communications
    GndStructureUtilityTelecommunicationsCommunications,
    /// Antenna Farm
    GndStructureUtilityTelecommunicationsCommunicationsAntennafarm,
    /// Satellite Communications
    GndStructureUtilityTelecommunicationsCommunicationsSatellitecommunications,
    /// Tower
    GndStructureUtilityTelecommunicationsCommunicationsTower,
    /// Radio
    GndStructureUtilityTelecommunicationsRadio,
    /// Station
    GndStructureUtilityTelecommunicationsRadioStation,
    /// Telephone
    GndStructureUtilityTelecommunicationsTelephone,
    /// Telephone Switching
    GndStructureUtilityTelecommunicationsTelephoneTelephoneswitching,
    /// Television
    GndStructureUtilityTelecommunicationsTelevision,
    /// Cable
    GndStructureUtilityTelecommunicationsTelevisionCable,
    /// Station
    GndStructureUtilityTelecommunicationsTelevisionStation,
    /// MEDICAL FACILITY
    GndStructureMedicalfacility,
    /// HOSPITAL
    GndStructureHospital,
    /// Hospital Care Facility
    GndStructureMedicalfacilityHospitalHospitalcarefacility,
    /// Road
    GndStructureRoad,
    /// Road (Highway)
    GndStructureRoadHighway,
    /// Road (Improved)
    GndStructureRoadImproved,
    /// Rest Area
    GndStructureRoadRestarea,
    /// Road (Un-improved)
    GndStructureRoadUnimproved,
    /// UNIT
    GndUnit,
    /// COMBAT
    GndCombat,
    /// ARMOR
    GndCombatArmor,
    /// ANTI ARMOR
    GndCombatArmorAntiarmor,
    /// ANTI ARMOR ARMORED
    GndCombatArmorAntiarmorarmored,
    /// ANTI ARMOR ARMORED AIR ASSAULT
    GndCombatArmorAntiarmorarmoredairassault,
    /// ANTI ARMOR ARMORED TRACKED
    GndCombatArmorAntiarmorarmoredtracked,
    /// ANTI ARMOR ARMORED WHEELED
    GndCombatArmorAntiarmorarmoredwheeled,
    /// ANTI ARMOR ARCTIC
    GndCombatArmorAntiarmorarctic,
    /// ANTI ARMOR DISMOUNTED
    GndCombatArmorAntiarmordismounted,
    /// ANTI ARMOR LIGHT
    GndCombatArmorAntiarmorlight,
    /// ANTI ARMOR AIRBORNE
    GndCombatArmorAntiarmorairborne,
    /// ANTI ARMOR MOTORIZED
    GndCombatArmorAntiarmormotorized,
    /// ANTI ARMOR MOTORIZED AIR ASSAULT
    GndCombatArmorAntiarmormotorizedairassault,
    /// ANTI ARMOR AIR ASSAULT
    GndCombatArmorAntiarmorairassault,
    /// ANTI ARMOR MOUNTAIN
    GndCombatArmorAntiarmormountain,
    /// ARMOR TRACK
    GndCombatArmorArmortrack,
    /// ARMOR TRACK AIRBORNE
    GndCombatArmorArmortrackairborne,
    /// ARMOR TRACK, HEAVY
    GndCombatArmorArmortrackHeavy,
    /// ARMOR TRACK, LIGHT
    GndCombatArmorArmortrackLight,
    /// ARMOR TRACK, MEDIUM
    GndCombatArmorArmortrackMedium,
    /// ARMOR TRACK, RECOVERY
    GndCombatArmorArmortrackRecovery,
    /// ARMOR TRACK AMPHIBIOUS
    GndCombatArmorArmortrackamphibious,
    /// ARMOR TRACK AMPHIBIOUS RECOVERY
    GndCombatArmorArmortrackamphibiousrecovery,
    /// ARMOR, WHEELED
    GndCombatArmorArmorWheeled,
    /// ARMOR, WHEELED AIRBORNE
    GndCombatArmorArmorWheeledairborne,
    /// ARMOR, WHEELED HEAVY
    GndCombatArmorArmorWheeledheavy,
    /// ARMOR, WHEELED LIGHT
    GndCombatArmorArmorWheeledlight,
    /// ARMOR, WHEELED MEDIUM
    GndCombatArmorArmorWheeledmedium,
    /// ARMOR, WHEELED RECOVERY
    GndCombatArmorArmorWheeledrecovery,
    /// ARMOR, WHEELED AIR ASSAULT
    GndCombatArmorArmorWheeledairassault,
    /// ARMOR, WHEELED AMPHIBIOUS
    GndCombatArmorArmorWheeledamphibious,
    /// ARMOR, WHEELED AMPHIBIOUS RECOVERY
    GndCombatArmorArmorWheeledamphibiousrecovery,
    /// AIR DEFENSE
    GndCombatDefenseAirdefense,
    /// COMPOSITE
    GndCombatDefenseComposite,
    /// GUN UNIT
    GndCombatDefenseGununit,
    /// H/MAD
    GndCombatDefenseHMad,
    /// HAWK
    GndCombatDefenseHawk,
    /// PATRIOT
    GndCombatDefensePatriot,
    /// AIR DEFENSE MISSILE
    GndCombatDefenseAirdefensemissile,
    /// AIR DEFENSE MISSILE HEAVY
    GndCombatDefenseAirdefensemissileheavy,
    /// AIR DEFENSE MISSILE LIGHT
    GndCombatDefenseAirdefensemissilelight,
    /// AIR DEFENSE MISSILE MOTORIZED (AVENGER)
    GndCombatDefenseAirdefensemissilemotorizedAvenger,
    /// AIR DEFENSE MISSILE MEDIUM
    GndCombatDefenseAirdefensemissilemedium,
    /// THEATER MISSILE DEFENSE UNIT
    GndCombatDefenseTheatermissiledefenseunit,
    /// SHORT RANGE
    GndCombatDefenseShortrange,
    /// STINGER
    GndCombatDefenseStinger,
    /// VULCAN
    GndCombatDefenseVulcan,
    /// TARGETING UNIT
    GndCombatDefenseTargetingunit,
    /// ENGINEER
    GndCombatEngineerEngineer,
    /// ENGINEER COMBAT
    GndCombatEngineerEngineercombat,
    /// ENGINEER COMBAT AIRBORNE
    GndCombatEngineerEngineercombatairborne,
    /// ENGINEER COMBAT ARCTIC
    GndCombatEngineerEngineercombatarctic,
    /// ENGINEER COMBAT HEAVY
    GndCombatEngineerEngineercombatheavy,
    /// ENGINEER COMBAT LIGHT (SAPPER)
    GndCombatEngineerEngineercombatlightSapper,
    /// ENGINEER COMBAT MEDIUM
    GndCombatEngineerEngineercombatmedium,
    /// ENGINEER COMBAT MOUNTAIN
    GndCombatEngineerEngineercombatmountain,
    /// ENGINEER COMBAT RECON
    GndCombatEngineerEngineercombatrecon,
    /// ENGINEER COMBAT AIR ASSAULT
    GndCombatEngineerEngineercombatairassault,
    /// ENGINEER COMBAT MECHANIZED (TRACK)
    GndCombatEngineerEngineercombatmechanizedTrack,
    /// ENGINEER COMBAT MOTORIZED
    GndCombatEngineerEngineercombatmotorized,
    /// ENGINEER CONSTRUCTION
    GndCombatEngineerEngineerconstruction,
    /// ENGINEER NAVAL CONSTRUCTION
    GndCombatEngineerEngineernavalconstruction,
    /// Artillery (Fixed)
    GndCombatArtilleryFixed,
    /// HOWITZER/GUN
    GndCombatHowitzerGun,
    /// AIRBORNE
    GndCombatAirborne,
    /// ARCTIC
    GndCombatArctic,
    /// Artillery (Mobile)
    GndCombatArtilleryMobile,
    /// HEAVY
    GndCombatHeavy,
    /// LIGHT
    GndCombatLight,
    /// MEDIUM
    GndCombatMedium,
    /// MOUNTAIN
    GndCombatMountain,
    /// AIR ASSAULT
    GndCombatAirassault,
    /// AMPHIBIOUS
    GndCombatAmphibious,
    /// MORTAR
    GndCombatMortar,
    /// AMPHIBIOUS MORTAR
    GndCombatAmphibiousmortar,
    /// SELF PROPELLED (SP) TRACKED MORTAR
    GndCombatSelfpropelledSpTrackedmortar,
    /// TOWED MORTAR
    GndCombatTowedmortar,
    /// TOWED AIRBORNE MORTAR
    GndCombatTowedairbornemortar,
    /// TOWED ARCTIC MORTAR
    GndCombatTowedarcticmortar,
    /// TOWED MOUNTAIN MORTAR
    GndCombatTowedmountainmortar,
    /// TOWED AIR ASSAULT MORTAR
    GndCombatTowedairassaultmortar,
    /// SP WHEELED MORTAR
    GndCombatSpwheeledmortar,
    /// METEOROLOGICAL
    GndCombatMeteorological,
    /// AIRBORNE METEOROLOGICAL
    GndCombatAirbornemeteorological,
    /// LIGHT METEOROLOGICAL
    GndCombatLightmeteorological,
    /// MOUNTAIN METEOROLOGICAL
    GndCombatMountainmeteorological,
    /// AIR ASSAULT METEOROLOGICAL
    GndCombatAirassaultmeteorological,
    /// ROCKET
    GndCombatRocket,
    /// Rockets (Fixed)
    GndCombatRocketsFixed,
    /// Rockets (Mobile)
    GndCombatRocketsMobile,
    /// MULTI ROCKET SELF PROPELLED
    GndCombatMultirocketselfpropelled,
    /// MULTI ROCKET TOWED
    GndCombatMultirockettowed,
    /// SINGLE ROCKET LAUNCHER
    GndCombatSinglerocketlauncher,
    /// SINGLE ROCKET TRUCK
    GndCombatSinglerockettruck,
    /// SINGLE ROCKET SELF PROPELLED
    GndCombatSinglerocketselfpropelled,
    /// SINGLE ROCKET TOWED
    GndCombatSinglerockettowed,
    /// ARTILLERY SURVEY
    GndCombatArtillerysurvey,
    /// TARGET ACQUISITION
    GndCombatTargetacquisition,
    /// ANGLICO
    GndCombatAnglico,
    /// COLT/FIST
    GndCombatColtFist,
    /// DISMOUNTED COLT/FIST
    GndCombatDismountedcoltFist,
    /// TRACKED COLT/FIST
    GndCombatTrackedcoltFist,
    /// FLASH (OPTICAL)
    GndCombatFlashOptical,
    /// RADAR
    GndCombatRadar,
    /// SOUND
    GndCombatSound,
    /// Troops (Open)
    GndCombatInfantryTroopsOpen,
    /// INFANTRY AIRBORNE
    GndCombatInfantryAirborne,
    /// INFANTRY ARCTIC
    GndCombatInfantryArctic,
    /// INFANTRY FIGHTING VEHICLE
    GndCombatInfantryFightingvehicle,
    /// INFANTRY LIGHT
    GndCombatInfantryLight,
    /// INFANTRY MOTORIZED
    GndCombatInfantryMotorized,
    /// INFANTRY NAVAL
    GndCombatInfantryNaval,
    /// INFANTRY MOUNTAIN
    GndCombatInfantryMountain,
    /// INFANTRY AIR ASSAULT
    GndCombatInfantryAirassault,
    /// INFANTRY MECHANIZED
    GndCombatInfantryMechanized,
    /// Troops (DugIn)
    GndCombatInfantryDugin,
    /// MISSILE (SURF SURF)
    GndCombatMissileSurfsurf,
    /// MISSILE (SURF SURF) STRATEGIC
    GndCombatMissileSurfsurfStrategic,
    /// MISSILE (SURF SURF) TACTICAL
    GndCombatMissileSurfsurfTactical,
    /// RECONNAISSANCE
    GndCombatRecon,
    /// RECONNAISSANCE AIRBORNE
    GndCombatReconAirborne,
    /// RECONNAISSANCE ARCTIC
    GndCombatReconArctic,
    /// RECONNAISSANCE HORSE
    GndCombatReconHorse,
    /// RECONNAISSANCE LIGHT
    GndCombatReconLight,
    /// RECONNAISSANCE MOUNTAIN
    GndCombatReconMountain,
    /// RECONNAISSANCE MARINE
    GndCombatReconMarine,
    /// RECONNAISSANCE MARINE DIVISION
    GndCombatReconMarinedivision,
    /// RECONNAISSANCE MARINE FORCE
    GndCombatReconMarineforce,
    /// RECONNAISSANCE MARINE LIGHT ARMORED
    GndCombatReconMarinelightarmored,
    /// RECONNAISSANCE AIR ASSAULT
    GndCombatReconAirassault,
    /// RECONNAISSANCE CAVALRY
    GndCombatReconCavalry,
    /// RECONNAISSANCE CAVALRY ARMORED
    GndCombatReconCavalryarmored,
    /// RECONNAISSANCE CAVALRY GROUND
    GndCombatReconCavalryground,
    /// RECONNAISSANCE CAVALRY MOTORIZED
    GndCombatReconCavalrymotorized,
    /// RECONNAISSANCE CAVALRY AIR
    GndCombatReconCavalryair,
    /// RECONNAISSANCE LONG RANGE SURVEILLANCE
    GndCombatReconLongrangesurveillance,
    /// INTERNAL SECURITY FORCES
    GndCombatSecurityInternalsecurityforces,
    /// AVIATION
    GndCombatSecurityAviation,
    /// GROUND
    GndCombatSecurityGround,
    /// MECHANIZED GROUND
    GndCombatSecurityMechanizedground,
    /// DISMOUNTED GROUND
    GndCombatSecurityDismountedground,
    /// MOTORIZED GROUND
    GndCombatSecurityMotorizedground,
    /// WHEELED MECHANIZED
    GndCombatSecurityWheeledmechanized,
    /// Rail
    GndCombatSecurityRail,
    /// RIVERINE
    GndCombatSecurityRiverine,
    /// AVIATION
    GndCombatAviationAviation,
    /// COMPOSITE
    GndCombatAviationComposite,
    /// FIXED WING
    GndCombatAviationFixedwing,
    /// ATTACK FIXED WING
    GndCombatAviationAttackfixedwing,
    /// RECON FIXED WING
    GndCombatAviationReconfixedwing,
    /// UTILITY FIXED WING
    GndCombatAviationUtilityfixedwing,
    /// ROTARY WING
    GndCombatAviationRotarywing,
    /// ATTACK ROTARY WING
    GndCombatAviationAttackrotarywing,
    /// MINE COUNTERMEASURE ROTARY WING
    GndCombatAviationMinecountermeasurerotarywing,
    /// SCOUT ROTARY WING
    GndCombatAviationScoutrotarywing,
    /// UTILITY ROTARY WING
    GndCombatAviationUtilityrotarywing,
    /// C2 ROTARY WING
    GndCombatAviationC2Rotarywing,
    /// MEDEVAC ROTARY WING
    GndCombatAviationMedevacrotarywing,
    /// HEAVY UTILITY ROTARY WING
    GndCombatAviationHeavyutilityrotarywing,
    /// LIGHT UTILITY ROTARY WING
    GndCombatAviationLightutilityrotarywing,
    /// MEDIUM UTILITY ROTARY WING
    GndCombatAviationMediumutilityrotarywing,
    /// ANTISUBMARINE WARFARE ROTARY WING
    GndCombatAviationAntisubmarinewarfarerotarywing,
    /// Helo ground unit (Dual)
    GndCombatAviationHeloDual,
    /// Helo ground unit (Single)
    GndCombatAviationHeloSingle,
    /// SEARCH AND RESCUE
    GndCombatAviationSearchandrescue,
    /// UNMANNED AERIAL VEHICLE
    GndCombatAviationUnmannedaerialvehicle,
    /// UNMANNED AERIAL VEHICLE FIXED WING
    GndCombatAviationUnmannedaerialvehiclefixedwing,
    /// UNMANNED AERIAL VEHICLE ROTARY WING
    GndCombatAviationUnmannedaerialvehiclerotarywing,
    /// VERTICAL/SHORT TAKEOFF AND LANDING
    GndCombatAviationVerticalShorttakeoffandlanding,
    /// SPECIAL C2 HEADQUARTERS COMPONENT
    GndSpecialc2Headquarterscomponent,
    /// COMBAT SERVICE SUPPORT
    GndCombatservicesupport,
    /// ADMINISTRATIVE (ADMIN)
    GndAdministrativeAdmin,
    /// ADMIN CORPS
    GndAdmincorps,
    /// FINANCE
    GndFinance,
    /// FINANCE CORPS
    GndFinancecorps,
    /// FINANCE THEATER
    GndFinancetheater,
    /// JUDGE ADVOCATE GENERAL (JAG)
    GndJudgeadvocategeneralJag,
    /// JAG CORPS
    GndJagcorps,
    /// JAG THEATER
    GndJagtheater,
    /// LABOR
    GndLabor,
    /// LABOR CORPS
    GndLaborcorps,
    /// LABOR THEATER
    GndLabortheater,
    /// MORTUARY/GRAVES REGISTRY
    GndMortuaryGravesregistry,
    /// MORTUARY/GRAVES REGISTRY CORPS
    GndMortuaryGravesregistrycorps,
    /// MORTUARY/GRAVES REGISTRY THEATER
    GndMortuaryGravesregistrytheater,
    /// POSTAL
    GndPostal,
    /// POSTAL CORPS
    GndPostalcorps,
    /// POSTAL THEATER
    GndPostaltheater,
    /// PUBLIC AFFAIRS
    GndPublicaffairs,
    /// PUBLIC AFFAIRS BROADCAST
    GndPublicaffairsbroadcast,
    /// PUBLIC AFFAIRS BROADCAST CORPS
    GndPublicaffairsbroadcastcorps,
    /// PUBLIC AFFAIRS BROADCAST THEATER
    GndPublicaffairsbroadcasttheater,
    /// PUBLIC AFFAIRS CORPS
    GndPublicaffairscorps,
    /// PUBLIC AFFAIRS JOINT INFORMATION BUREAU
    GndPublicaffairsjointinformationbureau,
    /// PUBLIC AFFAIRS JIB CORPS
    GndPublicaffairsjibcorps,
    /// PUBLIC AFFAIRS JIB THEATER
    GndPublicaffairsjibtheater,
    /// PUBLIC AFFAIRS THEATER
    GndPublicaffairstheater,
    /// QUARTERMASTER (SUPPLY)
    GndQuartermasterSupply,
    /// QUARTERMASTER (SUPPLY) CORPS
    GndQuartermasterSupplyCorps,
    /// QUARTERMASTER (SUPPLY) THEATER
    GndQuartermasterSupplyTheater,
    /// RELIGIOUS/CHAPLAIN
    GndReligiousChaplain,
    /// RELIGIOUS/CHAPLAIN CORPS
    GndReligiousChaplaincorps,
    /// RELIGIOUS/CHAPLAIN THEATER
    GndReligiousChaplaintheater,
    /// PERSONNEL SERVICES
    GndPersonnelservices,
    /// PERSONNEL CORPS
    GndPersonnelcorps,
    /// PERSONNEL THEATER
    GndPersonneltheater,
    /// ADMIN THEATER
    GndAdmintheater,
    /// MORALE, WELFARE, RECREATION (MWR)
    GndMoraleWelfareRecreationMwr,
    /// MWR CORPS
    GndMwrcorps,
    /// MWR THEATER
    GndMwrtheater,
    /// REPLACEMENT HOLDING UNIT (RHU)
    GndReplacementholdingunitRhu,
    /// RHU CORPS
    GndRhucorps,
    /// RHU THEATER
    GndRhutheater,
    /// MEDICAL
    GndMedical,
    /// MEDICAL CORPS
    GndMedicalcorps,
    /// MEDICAL DENTAL
    GndMedicaldental,
    /// MEDICAL DENTAL CORPS
    GndMedicaldentalcorps,
    /// MEDICAL DENTAL THEATER
    GndMedicaldentaltheater,
    /// MEDICAL TREATMENT FACILITY
    GndMedicaltreatmentfacility,
    /// MEDICAL TREATMENT FACILITY CORPS
    GndMedicaltreatmentfacilitycorps,
    /// MEDICAL TREATMENT FACILITY THEATER
    GndMedicaltreatmentfacilitytheater,
    /// MEDICAL PSYCHOLOGICAL
    GndMedicalpsychological,
    /// MEDICAL PSYCHOLOGICAL CORPS
    GndMedicalpsychologicalcorps,
    /// MEDICAL PSYCHOLOGICAL THEATER
    GndMedicalpsychologicaltheater,
    /// MEDICAL THEATER
    GndMedicaltheater,
    /// MEDICAL VETERINARY
    GndMedicalveterinary,
    /// MEDICAL VETERINARY CORPS
    GndMedicalveterinarycorps,
    /// MEDICAL VETERINARY THEATER
    GndMedicalveterinarytheater,
    /// SUPPLY
    GndSupply,
    /// SUPPLY CLASS I
    GndSupplyclassi,
    /// SUPPLY CLASS I CORPS
    GndSupplyclassicorps,
    /// SUPPLY CLASS I THEATER
    GndSupplyclassitheater,
    /// SUPPLY CLASS II
    GndSupplyclassii,
    /// SUPPLY CLASS II CORPS
    GndSupplyclassiicorps,
    /// SUPPLY CLASS II THEATER
    GndSupplyclassiitheater,
    /// SUPPLY CLASS III
    GndSupplyclassiii,
    /// SUPPLY CLASS III AVIATION
    GndSupplyclassiiiaviation,
    /// SUPPLY CLASS III AVIATION CORPS
    GndSupplyclassiiiaviationcorps,
    /// SUPPLY CLASS III AVIATION THEATER
    GndSupplyclassiiiaviationtheater,
    /// SUPPLY CLASS III CORPS
    GndSupplyclassiiicorps,
    /// SUPPLY CLASS III THEATER
    GndSupplyclassiiitheater,
    /// SUPPLY CLASS IV
    GndSupplyclassiv,
    /// SUPPLY CLASS IV CORPS
    GndSupplyclassivcorps,
    /// SUPPLY CLASS IV THEATER
    GndSupplyclassivtheater,
    /// SUPPLY CLASS V
    GndSupplyclassv,
    /// SUPPLY CLASS V CORPS
    GndSupplyclassvcorps,
    /// SUPPLY CLASS V THEATER
    GndSupplyclassvtheater,
    /// SUPPLY CLASS VI
    GndSupplyclassvi,
    /// SUPPLY CLASS VI CORPS
    GndSupplyclassvicorps,
    /// SUPPLY CLASS VI THEATER
    GndSupplyclassvitheater,
    /// SUPPLY CLASS VII
    GndSupplyclassvii,
    /// SUPPLY CLASS VII CORPS
    GndSupplyclassviicorps,
    /// SUPPLY CLASS VII THEATER
    GndSupplyclassviitheater,
    /// SUPPLY CLASS VIII
    GndSupplyclassviii,
    /// SUPPLY CLASS VIII CORPS
    GndSupplyclassviiicorps,
    /// SUPPLY CLASS VIII THEATER
    GndSupplyclassviiitheater,
    /// SUPPLY CLASS IX
    GndSupplyclassix,
    /// SUPPLY CLASS IX CORPS
    GndSupplyclassixcorps,
    /// SUPPLY CLASS IX THEATER
    GndSupplyclassixtheater,
    /// SUPPLY CORPS
    GndSupplycorps,
    /// SUPPLY LAUNDRY/BATH
    GndSupplylaundryBath,
    /// SUPPLY LAUNDRY/BATH CORPS
    GndSupplylaundryBathcorps,
    /// SUPPLY LAUNDRY/BATH THEATER
    GndSupplylaundryBaththeater,
    /// SUPPLY THEATER
    GndSupplytheater,
    /// SUPPLY WATER
    GndSupplywater,
    /// SUPPLY WATER CORPS
    GndSupplywatercorps,
    /// SUPPLY WATER PURIFICATION
    GndSupplywaterpurification,
    /// SUPPLY WATER PURIFICATION CORPS
    GndSupplywaterpurificationcorps,
    /// SUPPLY WATER PURIFICATION THEATER
    GndSupplywaterpurificationtheater,
    /// SUPPLY WATER THEATER
    GndSupplywatertheater,
    /// SUPPLY CLASS X
    GndSupplyclassx,
    /// SUPPLY CLASS X CORPS
    GndSupplyclassxcorps,
    /// SUPPLY CLASS X THEATER
    GndSupplyclassxtheater,
    /// SupplyDepot
    GndSupplydepot,
    /// TRANSPORTATION
    GndTransportation,
    /// APOD/APOE
    GndApodApoe,
    /// APOD/APOE CORPS
    GndApodApoecorps,
    /// APOD/APOE THEATER
    GndApodApoetheater,
    /// TRANSPORTATION CORPS
    GndTransportationcorps,
    /// MISSILE
    GndMissile,
    /// MISSILE CORPS
    GndMissilecorps,
    /// MISSILE THEATER
    GndMissiletheater,
    /// MOVEMENT CONTROL CENTER(MCC)
    GndMovementcontrolcenterMcc,
    /// MCC CORPS
    GndMcccorps,
    /// MCC THEATER
    GndMcctheater,
    /// RAILHEAD
    GndRailhead,
    /// RAILHEAD CORPS
    GndRailheadcorps,
    /// RAILHEAD THEATER
    GndRailheadtheater,
    /// SPOD/SPOE
    GndSpodSpoe,
    /// SPOD/SPOE CORPS
    GndSpodSpoecorps,
    /// SPOD/SPOE THEATER
    GndSpodSpoetheater,
    /// TRANSPORTATION THEATER
    GndTransportationtheater,
    /// MAINTENANCE
    GndMaintenance,
    /// MAINTENANCE CORPS
    GndMaintenancecorps,
    /// ELECTRO OPTICAL
    GndElectrooptical,
    /// ELECTRO OPTICAL CORPS
    GndElectroopticalcorps,
    /// ELECTRO OPTICAL THEATER
    GndElectroopticaltheater,
    /// MAINTENANCE HEAVY
    GndMaintenanceheavy,
    /// MAINTENANCE HEAVY CORPS
    GndMaintenanceheavycorps,
    /// MAINTENANCE HEAVY THEATER
    GndMaintenanceheavytheater,
    /// ORDNANCE
    GndOrdnance,
    /// ORDNANCE CORPS
    GndOrdnancecorps,
    /// ORDNANCE MISSILE
    GndOrdnancemissile,
    /// ORDNANCE MISSILE CORPS
    GndOrdnancemissilecorps,
    /// ORDNANCE MISSILE THEATER
    GndOrdnancemissiletheater,
    /// ORDNANCE THEATER
    GndOrdnancetheater,
    /// MAINTENANCE RECOVERY
    GndMaintenancerecovery,
    /// MAINTENANCE RECOVERY CORPS
    GndMaintenancerecoverycorps,
    /// MAINTENANCE RECOVERY THEATER
    GndMaintenancerecoverytheater,
    /// MAINTENANCE THEATER
    GndMaintenancetheater,
    /// COMBAT SUPPORT
    GndCombatsupport,
    /// COMBAT SUPPORT NBC
    GndCombatsupportnbc,
    /// BIOLOGICAL
    GndBiological,
    /// RECON EQUIPPED
    GndReconequipped,
    /// CHEMICAL
    GndChemical,
    /// SMOKE/DECON
    GndSmokeDecon,
    /// MECHANIZED SMOKE/DECON
    GndMechanizedsmokeDecon,
    /// MOTORIZED SMOKE/DECON
    GndMotorizedsmokeDecon,
    /// CHEMICAL RECON
    GndChemicalrecon,
    /// CHEMICAL WHEELED ARMORED VEHICLE
    GndChemicalwheeledarmoredvehicle,
    /// SMOKE
    GndSmoke,
    /// ARMOR SMOKE
    GndArmorsmoke,
    /// MOTORIZED SMOKE
    GndMotorizedsmoke,
    /// DECONTAMINATION
    GndDecontamination,
    /// NUCLEAR
    GndNuclear,
    /// EXPLOSIVE ORDINANCE DISPOSAL
    GndExplosiveordinancedisposal,
    /// INFORMATION WARFARE UNIT
    GndInformationwarfareunit,
    /// LAW ENFORCEMENT UNIT
    GndLawenforcementunit,
    /// CIVILIAN LAW ENFORCEMENT
    GndCivilianlawenforcement,
    /// CENTRAL INTELLIGENCE DIVISION (CID)
    GndCentralintelligencedivisionCid,
    /// SECURITY POLICE (AIR)
    GndSecuritypoliceAir,
    /// MILITARY POLICE
    GndMilitarypolice,
    /// SHORE PATROL
    GndShorepatrol,
    /// MILITARY INTELLIGENCE
    GndMilitaryintelligence,
    /// AERIAL EXPLOITATION
    GndAerialexploitation,
    /// COUNTER INTELLIGENCE
    GndCounterintelligence,
    /// JOINT INTELLIGENCE CENTER
    GndJointintelligencecenter,
    /// METEOROLOGICAL
    GndMeteorological,
    /// OPERATIONS
    GndOperations,
    /// INTERROGATION
    GndInterrogation,
    /// SURVEILLANCE
    GndSurveillance,
    /// GROUND SURVEILLANCE RADAR
    GndGroundsurveillanceradar,
    /// SENSOR
    GndSensor,
    /// SENSOR SCM
    GndSensorscm,
    /// GROUND STATION MODULE
    GndGroundstationmodule,
    /// SIGNAL INTELLIGENCE (SIGINT)
    GndSignalintelligenceSigint,
    /// ELECTRONIC WARFARE
    GndElectronicwarfare,
    /// ARMORED WHEELED VEHICLE
    GndArmoredwheeledvehicle,
    /// CORPS
    GndCorps,
    /// DIRECTION FINDING
    GndDirectionfinding,
    /// INTERCEPT
    GndIntercept,
    /// JAMMING
    GndJamming,
    /// THEATER
    GndTheater,
    /// TACTICAL EXPLOIT
    GndTacticalexploit,
    /// LANDING SUPPORT
    GndLandingsupport,
    /// SIGNAL UNIT
    GndSignalunit,
    /// AREA
    GndArea,
    /// COMMUNICATION CONFIGURED PACKAGE
    GndCommunicationconfiguredpackage,
    /// LARGE COMMUNICATION CONFIGURED PACKAGE
    GndLargecommunicationconfiguredpackage,
    /// FORWARD COMMUNICATIONS
    GndForwardcommunications,
    /// MULTIPLE SUBSCRIBER ELEMENT
    GndMultiplesubscriberelement,
    /// LARGE EXTENSION NODE
    GndLargeextensionnode,
    /// NODE CENTER
    GndNodecenter,
    /// SMALL EXTENSION NODE
    GndSmallextensionnode,
    /// COMMAND OPERATIONS
    GndCommandoperations,
    /// RADIO UNIT
    GndRadiounit,
    /// TACTICAL SATELLITE
    GndTacticalsatellite,
    /// TELETYPE CENTER
    GndTeletypecenter,
    /// RELAY
    GndRelay,
    /// SIGNAL SUPPORT
    GndSignalsupport,
    /// TELEPHONE SWITCH
    GndTelephoneswitch,
    /// ELECTRONIC RANGING
    GndElectronicranging,
    /// Incident Management Resources
    GndImresources,
    /// Animal Health
    GndImresourcesAnimalhealth,
    /// Large Animal Rescue Strike Team
    GndLargeanimalrescuestriketeam,
    /// Large Animal Sheltering Team
    GndLargeanimalshelteringteam,
    /// Large Animal Transport Team
    GndLargeanimaltransportteam,
    /// Small Animal Rescue Strike Team
    GndSmallanimalrescuestriketeam,
    /// Small Animal Sheltering Team
    GndSmallanimalshelteringteam,
    /// Small Animal Transport Team
    GndSmallanimaltransportteam,
    /// Incident Management Team Animal Protection
    GndIncidentmanagementteamanimalprotection,
    /// Emergency Management
    GndImresourcesEmergencymanagement,
    /// Airborne Communications Relay(CAP)
    GndAirbornecommunicationsrelayCap,
    /// Airborne Communications Relay Team
    GndAirbornecommunicationsrelayteam,
    /// Airborne Transport Team
    GndAirbornetransportteam,
    /// Critical Incident Stress Management Team
    GndCriticalincidentstressmanagementteam,
    /// Communications Support Team
    GndCommunicationssupportteam,
    /// Donations Coordinator
    GndDonationscoordinator,
    /// Donations Management Personnel Team
    GndDonationsmanagementpersonnelteam,
    /// Evacuation Coordination Team
    GndEvacuationcoordinationteam,
    /// EOC Finance Administration
    GndEocfinanceadministration,
    /// Evacuation Liaison Team
    GndEvacuationliaisonteam,
    /// EOC Management Support Team
    GndEocmanagementsupportteam,
    /// EOC Operations Section Chief
    GndEocoperationssectionchief,
    /// EOC Planning Section Chief
    GndEocplanningsectionchief,
    /// Individual Assistance Disaster Assessment Team Leader
    GndIndividualassistancedisasterassessmentteamleader,
    /// Individual Assistance Disaster Assessment Team
    GndIndividualassistancedisasterassessmentteam,
    /// Incident Management Team
    GndIncidentmanagementteam,
    /// Mobile Communications Center
    GndMobilecommunicationscenter,
    /// Mobile Feeding Kitchen
    GndMobilefeedingkitchen,
    /// Public Assistance Coordinator
    GndPublicassistancecoordinator,
    /// Rapid Needs Assessment Team
    GndRapidneedsassessmentteam,
    /// Shelter Management Team
    GndSheltermanagementteam,
    /// Volunteer Agency Liaison
    GndVolunteeragencyliaison,
    /// Emergency Medical Services
    GndImresourcesEmergencymedicalservices,
    /// Air Ambulance (Fixed-Wing)
    GndAirambulanceFixedWing,
    /// Air Ambulance (Rotary-Wing)
    GndAirambulanceRotaryWing,
    /// Ambulances (Ground)
    GndAmbulancesGround,
    /// Ambulance Strike Team
    GndAmbulancestriketeam,
    /// Ambulance Task Force
    GndAmbulancetaskforce,
    /// Emergency Medical Task Force
    GndEmergencymedicaltaskforce,
    /// Fire HAZMAT
    GndImresourcesFirehazmat,
    /// Area Command Team, Firefighting
    GndAreacommandteamFirefighting,
    /// Brush Patrol
    GndBrushpatrol,
    /// Crew Transport
    GndCrewtransport,
    /// Engine, Fire (Pumper)
    GndEngineFirePumper,
    /// Fire Boat
    GndFireboat,
    /// Fuel Tender
    GndFueltender,
    /// Fire Truck
    GndFiretruck,
    /// Foam Tender
    GndFoamtender,
    /// Hand Crew
    GndHandcrew,
    /// HazMat Entry Team
    GndHazmatentryteam,
    /// Helicopters, Firefighting
    GndHelicoptersfirefighting,
    /// Helitanker
    GndHelitanker,
    /// Incident Management Team Firefighting
    GndIncidentmanagementteamfirefighting,
    /// Interagency Buying Team Firefighting
    GndInteragencybuyingteamfirefighting,
    /// Mobile Communications Unit
    GndMobilecommunicationsunit,
    /// USCG National Strike Force
    GndUscgnationalstrikeforce,
    /// Portable Pump
    GndPortablepump,
    /// Strike Team, Engine (Fire)
    GndStriketeamEngineFire,
    /// Water Tender, Firefighting (Tanker)
    GndWatertenderFirefightingTanker,
    /// Health and Medical
    GndImresourcesHealthandmedical,
    /// Disaster Medical Assistance Team - Basic
    GndDisastermedicalassistanceteamBasic,
    /// Disaster Medical Assistance Team - Crush Injury Specialty
    GndDisastermedicalassistanceteamCrushinjuryspecialty,
    /// Disaster Medical Assistance Team - Mental Health
    GndDisastermedicalassistanceteamMentalhealth,
    /// Disaster Medical Assistance Team - Pediatric
    GndDisastermedicalassistanceteamPediatric,
    /// Disaster Medical Assistance Team - Burn
    GndDisastermedicalassistanceteamBurn,
    /// Disaster Mortuary Operational Response Team
    GndDisastermortuaryoperationalresponseteam,
    /// International Medical Surgical Response Team
    GndInternationalmedicalsurgicalresponseteam,
    /// NDMS Management Support Team
    GndNdmsmanagementsupportteam,
    /// Veterinary Medical Assistance Team
    GndVeterinarymedicalassistanceteam,
    /// Law Enforcement
    GndImresourcesLawenforcement,
    /// Bomb Squad Explosives Team
    GndBombsquadexplosivesteam,
    /// Crowd Control Team
    GndCrowdcontrolteam,
    /// Public Safety Dive Team
    GndPublicsafetydiveteam,
    /// Aviation-Helicopters Patrol Surveillance
    GndAviationHelicopterspatrolsurveillance,
    /// Observation Aircraft
    GndObservationaircraft,
    /// SWAT Tactical Team
    GndSwattacticalteam,
    /// Public Works
    GndImresourcesPublicworks,
    /// Air Curtain Burners (Above Ground)
    GndAircurtainburnersAboveground,
    /// Air Conditioner Heater
    GndAirconditionerheater,
    /// Air Curtain Burners (In Ground)
    GndAircurtainburnersInground,
    /// All Terrain Cranes
    GndAllterraincranes,
    /// Backhoe Loader
    GndBackhoeloader,
    /// Chillers Air Handlers
    GndChillersairhandlers,
    /// Crawler Cranes
    GndCrawlercranes,
    /// Concrete Cutter Multi-Processor for Hydraulic Excavator
    GndConcretecuttermultiProcessorforhydraulicexcavator,
    /// Disaster Assessment Team
    GndDisasterassessmentteam,
    /// Debris Management Monitoring Team
    GndDebrismanagementmonitoringteam,
    /// Debris Management Site Reduction Team
    GndDebrismanagementsitereductionteam,
    /// Debris Management Team
    GndDebrismanagementteam,
    /// Disaster Recovery Team
    GndDisasterrecoveryteam,
    /// Dump Trailer
    GndDumptrailer,
    /// Dump Truck Off Road
    GndDumptruckoffroad,
    /// Dump Truck On Road
    GndDumptruckonroad,
    /// Electrical Power Restoration Team
    GndElectricalpowerrestorationteam,
    /// Engineering Services
    GndEngineeringservices,
    /// Flat Bed Trailer Truck
    GndFlatbedtrailertruck,
    /// Generators
    GndGenerators,
    /// Hydraulic Excavator (Large)
    GndHydraulicexcavatorLarge,
    /// Hydraulic Excavator (Medium)
    GndHydraulicexcavatorMedium,
    /// Hydraulic Truck Cranes
    GndHydraulictruckcranes,
    /// Lattice Truck Cranes
    GndLatticetruckcranes,
    /// Tug Boat
    GndTugboat,
    /// Track Dozer
    GndTrackdozer,
    /// Tub Grinder
    GndTubgrinder,
    /// Tractor Trailer
    GndTractortrailer,
    /// Wheel Dozer
    GndWheeldozer,
    /// Wheel Loaders (Large)
    GndWheelloadersLarge,
    /// Wheel Loaders (Medium)
    GndWheelloadersMedium,
    /// Wheel Loaders (Small)
    GndWheelloadersSmall,
    /// Water Purification Team
    GndWaterpurificationteam,
    /// Water Truck
    GndWatertruck,
    /// Search and Rescue
    GndImresourcesSearchandrescue,
    /// Airborne Reconnaissance
    GndAirbornereconnaissance,
    /// Air Search Team
    GndAirsearchteam,
    /// Canine SAR Team Avalanche Snow
    GndCaninesarteamavalanchesnow,
    /// Cave SAR Team
    GndCavesarteam,
    /// Canine SAR Team Disaster Response
    GndCaninesarteamdisasterresponse,
    /// Canine SAR Team Land Cadaver
    GndCaninesarteamlandcadaver,
    /// Collapse SAR Team
    GndCollapsesarteam,
    /// Canine SAR Team Water
    GndCaninesarteamwater,
    /// Canine SAR Team Wilderness
    GndCaninesarteamwilderness,
    /// Canine SAR Team Wilderness Tracking
    GndCaninesarteamwildernesstracking,
    /// Mine Tunnel SAR Team
    GndMinetunnelsarteam,
    /// Mountain SAR Team
    GndMountainsarteam,
    /// Radio Direction Finding Team
    GndRadiodirectionfindingteam,
    /// Swiftwater Flood Search and Dive Rescue Team
    GndSwiftwaterfloodsearchanddiverescueteam,
    /// USAR Incident Support Team
    GndUsarincidentsupportteam,
    /// USAR Task Force
    GndUsartaskforce,
    /// Wilderness SAR Team
    GndWildernesssarteam,
    /// Other
    GndImresourcesOther,
    /// SEA SURFACE TRACK
    SurfaceSeasurfacetrack,
    /// COMBATANT
    SurfaceCombatant,
    /// AMPHIBIOUS WARFARE SHIP
    SurfaceAmphibiouswarfareship,
    /// ASSAULT VESSEL
    SurfaceAssaultvessel,
    /// LANDING CRAFT
    SurfaceLandingcraft,
    /// LANDING SHIP
    SurfaceLandingship,
    /// HOVERCRAFT
    SurfaceHovercraft,
    /// LINE
    SurfaceLine,
    /// BATTLESHIP
    SurfaceBattleship,
    /// CRUISER
    SurfaceCruiser,
    /// CARRIER
    SurfaceCarrier,
    /// DESTROYER
    SurfaceDestroyer,
    /// FRIGATE/CORVETTE
    SurfaceFrigateCorvette,
    /// MINE WARFARE VESSEL
    SurfaceMinewarfarevessel,
    /// MCM SUPPORT
    SurfaceMcmsupport,
    /// MCM DRONE
    SurfaceMcmdrone,
    /// MINEHUNTER
    SurfaceMinehunter,
    /// MINELAYER
    SurfaceMinelayer,
    /// MINESWEEPER
    SurfaceMinesweeper,
    /// PATROL
    SurfacePatrol,
    /// ANTISUBMARINE WARFARE
    SurfaceAntisubmarinewarfare,
    /// ANTISURFACE WARFARE
    SurfaceAntisurfacewarfare,
    /// NAVY GROUP
    SurfaceNavygroup,
    /// CONVOY
    SurfaceConvoy,
    /// NAVY TASK GROUP
    SurfaceNavytaskgroup,
    /// NAVY TASK FORCE
    SurfaceNavytaskforce,
    /// NAVY TASK UNIT
    SurfaceNavytaskunit,
    /// NONCOMBATANT
    SurfaceNoncombatant,
    /// FLEET SUPPORT
    SurfaceFleetsupport,
    /// INTELLIGENCE
    SurfaceIntelligence,
    /// HOSPITAL SHIP
    SurfaceHospitalship,
    /// STATION
    SurfaceStation,
    /// RESCUE
    SurfaceRescue,
    /// UNDERWAY REPLENISHMENT
    SurfaceUnderwayreplenishment,
    /// SERVICE/SUPPORT HARBOR
    SurfaceServiceSupportharbor,
    /// OWN TRACK
    SurfaceOwntrack,
    /// ASW SHIP
    SurfaceAswship,
    /// PICKET
    SurfacePicket,
    /// NON MILITARY
    SurfaceNonmilitary,
    /// FISHING
    SurfaceFishing,
    /// DRIFTER
    SurfaceDrifter,
    /// DREDGE
    SurfaceDredge,
    /// TRAWLER
    SurfaceTrawler,
    /// LAW ENFORCEMENT VESSEL
    SurfaceLawenforcementvessel,
    /// MERCHANT
    SurfaceMerchant,
    /// CARGO
    SurfaceCargo,
    /// FERRY
    SurfaceFerry,
    /// HAZARDOUS MATERIALS (HAZMAT)
    SurfaceHazardousmaterialsHazmat,
    /// OILER/TANKER
    SurfaceOilerTanker,
    /// PASSENGER
    SurfacePassenger,
    /// ROLL ON/ROLL OFF
    SurfaceRollonRolloff,
    /// TOWING VESSEL
    SurfaceTowingvessel,
    /// TUG
    SurfaceTug,
    /// LEISURE CRAFT
    SurfaceLeisurecraft,
    /// SUBSURFACE TRACK
    SubsurfSubsurfacetrack,
    /// NON SUBMARINE
    SubsurfNonsubmarine,
    /// DIVER
    SubsurfDiver,
    /// SUBMARINE
    SubsurfSubmarine,
    /// SUBMARINE CONVENTIONAL PROPULSION
    SubsurfSubmarineconventionalpropulsion,
    /// SUBMARINE NUCLEAR PROPULSION
    SubsurfSubmarinenuclearpropulsion,
    /// OTHER SUBMERSIBLE
    SubsurfOthersubmersible,
    /// STATION
    SubsurfStation,
    /// ASW SUBMARINE
    SubsurfAswsubmarine,
    /// UNMANNED UNDERWATER VEHICLE (UUV)
    SubsurfUnmannedunderwatervehicleUuv,
    /// UNDERWATER WEAPON
    SubsurfUnderwaterweapon,
    /// UNDERWATER DECOY
    SubsurfUnderwaterdecoy,
    /// SEA MINE DECOY
    SubsurfSeaminedecoy,
    /// SEA MINE
    SubsurfSeamine,
    /// SEA MINE DEALT
    SubsurfSeaminedealt,
    /// SEA MINE (FLOATING)
    SubsurfSeamineFloating,
    /// SEA MINE (FLOATING) DEALT
    SubsurfSeamineFloatingDealt,
    /// SEA MINE (GROUND)
    SubsurfSeamineGround,
    /// SEA MINE (GROUND) DEALT
    SubsurfSeamineGroundDealt,
    /// SEA MINE (MOORED)
    SubsurfSeamineMoored,
    /// SEA MINE (MOORED) DEALT
    SubsurfSeamineMooredDealt,
    /// SEA MINE (IN OTHER POSITION)
    SubsurfSeamineInotherposition,
    /// SEA MINE (IN OTHER POSITION) DEALT
    SubsurfSeamineInotherpositionDealt,
    /// TORPEDO
    SubsurfTorpedo,
    /// Other
    Other,
    /// Incident
    OtherIncident,
    /// Geophysical
    OtherIncidentGeo,
    /// Geophysical Avalanche
    OtherIncidentGeoAvalanche,
    /// Geophysical Earthquake
    OtherIncidentGeoEarthquake,
    /// Geophysical Earthquake Aftershock
    OtherIncidentGeoAftershock,
    /// Geophysical Earthquake Epicenter
    OtherIncidentGeoEpicenter,
    /// Geophysical Landslide
    OtherIncidentGeoLandslide,
    /// Geophysical Subsistance
    OtherIncidentGeoSubsistance,
    /// Geophysical Volcano
    OtherIncidentGeoVolcano,
    /// Geophysical Volcano Eruption
    OtherIncidentGeoEruption,
    /// Geophysical Volcano Threat
    OtherIncidentGeoVolcanothreat,
    /// Meteorological
    OtherIncidentMet,
    /// Meteorological Drizzle
    OtherIncidentMetDrizzle,
    /// Meteorological Drought
    OtherIncidentMetDrought,
    /// Meteorological Flood
    OtherIncidentMetFlood,
    /// Meteorological Fog
    OtherIncidentMetFog,
    /// Meteorological Hail
    OtherIncidentMetHail,
    /// Meteorological Inversion
    OtherIncidentMetInversion,
    /// Meteorological Rain
    OtherIncidentMetRain,
    /// Meteorological Sandstorm
    OtherIncidentMetSandstorm,
    /// Meteorological Snow
    OtherIncidentMetSnow,
    /// Meteorological Thunderstorm
    OtherIncidentMetthunderstorm,
    /// Meteorological Tornado
    OtherIncidentMetTornado,
    /// Meteorological Cyclone
    OtherIncidentMetCyclone,
    /// Meteorological Tsunami
    OtherIncidentMetTsunami,
    /// Safety
    OtherIncidentSafety,
    /// Security
    OtherIncidentSecurity,
    /// Security Civil Disturbance
    OtherIncidentSecurityCivildisturb,
    /// Security Civil Disturbance Demonstration
    OtherIncidentSecurityCivildisturbDemonstration,
    /// Security Civil Disturbance Displaced Population
    OtherIncidentSecurityCivildisturbDisplaced,
    /// Security Civil Disturbance Rioting
    OtherIncidentSecurityCivildisturbRioting,
    /// Security Law Enforcement Activity
    OtherIncidentSecurityLawenforcement,
    /// Security Law Enforcement Activity Bombing
    OtherIncidentSecurityLawenforcementBombing,
    /// Security Law Enforcement Activity Bombing Explosion
    OtherIncidentSecurityLawenforcementBombexplosion,
    /// Security Law Enforcement Activity Bombing Threat
    OtherIncidentSecurityLawenforcementbombthreat,
    /// Security Law Enforcement Activity Looting
    OtherIncidentSecurityLawenforcementLooting,
    /// Security Law Enforcement Activity Poisoning
    OtherIncidentSecurityLawenforcementPoisoning,
    /// Security Law Enforcement Activity Shooting
    OtherIncidentSecurityLawenforcementShooting,
    /// Rescue
    OtherIncidentRescue,
    /// Incident Fire
    OtherIncidentFire,
    /// Incident Fire Wild
    OtherIncidentFireWild,
    /// Incident Fire Non-residential
    OtherIncidentFireNonresidential,
    /// Incident Fire Non-residential Special Need
    OtherIncidentFireNonresidentialspecialneed,
    /// Incident Fire Non-residential School
    OtherIncidentFireNonresidentialschool,
    /// Incident Fire Residential
    OtherIncidentFireResidential,
    /// Incident Fire Hot Spot
    OtherIncidentFireHotspot,
    /// Incident Fire Origin
    OtherIncidentFireOrigin,
    /// Incident Fire Smoke
    OtherIncidentFireSmoke,
    /// Incident Fire Special Needs
    OtherIncidentFireSpecialneeds,
    /// Medical and Public Health
    OtherIncidentMedicalpublichealth,
    /// Pollution and other Environmental
    OtherIncidentPollutionenvironmental,
    /// Public and Private transportation
    OtherIncidentTransportation,
    /// Public and Private transportation Air Incident
    OtherIncidentTransportationAir,
    /// Public and Private transportation Air Accident
    OtherIncidentTransportationAiraccident,
    /// Public and Private transportation Air Hijacking
    OtherIncidentTransportationAirhijacking,
    /// Public and Private transportation Maritime Incident
    OtherIncidentTransportationMaritime,
    /// Public and Private transportation Maritime Accident
    OtherIncidentTransportationMaritimeaccident,
    /// Public and Private transportation Maritime Hijacking
    OtherIncidentTransportationMaritimehijacking,
    /// Public and Private transportation Rail Incident
    OtherIncidentTransportationRail,
    /// Public and Private transportation Rail Accident
    OtherIncidentTransportationRailaccident,
    /// Public and Private transportation Rail Hijacking
    OtherIncidentTransportationRailhijacking,
    /// Public and Private transportation Vehicle Incident
    OtherIncidentTransportationVehicle,
    /// Public and Private transportation Vehicle Accident
    OtherIncidentTransportationVehicleaccident,
    /// Public and Private transportation Vehicle Hijacking
    OtherIncidentTransportationVehiclehijacking,
    /// Infrastructure
    OtherIncidentInfrastructure,
    /// CBRNE
    OtherIncidentCbrne,
    /// Other
    OtherIncidentOther,
    /// MAYDAY
    Maydaymessage,
}

impl CoTType {
    pub fn to_string(&self) -> &'static str {
        match self {
            CoTType::AirAirtrack => "a-.-A",
            CoTType::AirCiv => "a-.-A-C",
            CoTType::AirCivFixed => "a-.-A-C-F",
            CoTType::AirCivFixedRpvDroneUav => "a-.-A-C-F-q",
            CoTType::AirCivRotary => "a-.-A-C-H",
            CoTType::AirCivBlimp => "a-.-A-C-L",
            CoTType::AirMilFixed => "a-.-A-M-F",
            CoTType::AirMilFixedGunship => "a-.-A-M-F-g",
            CoTType::AirMilFixedAttackStrike => "a-.-A-M-F-A",
            CoTType::AirMilFixedBomber => "a-.-A-M-F-B",
            CoTType::AirMilFixedTransport => "a-.-A-M-F-C",
            CoTType::AirMilFixedTransportHeavy => "a-.-A-M-F-C-H",
            CoTType::AirMilFixedTransportLight => "a-.-A-M-F-C-L",
            CoTType::AirMilFixedTransportMedium => "a-.-A-M-F-C-M",
            CoTType::AirMilFixedC2 => "a-.-A-M-F-D",
            CoTType::AirMilFixedFighter => "a-.-A-M-F-F",
            CoTType::AirMilFixedInterceptor => "a-.-A-M-F-F-I",
            CoTType::AirMilFixedCsar => "a-.-A-M-F-H",
            CoTType::AirMilFixedEcmJammer => "a-.-A-M-F-J",
            CoTType::AirMilFixedTanker => "a-.-A-M-F-K",
            CoTType::AirMilFixedVstol => "a-.-A-M-F-L",
            CoTType::AirMilFixedSof => "a-.-A-M-F-M",
            CoTType::AirMilFixedMedevac => "a-.-A-M-F-O",
            CoTType::AirMilFixedPatrol => "a-.-A-M-F-P",
            CoTType::AirMilFixedMinecountermeasures => "a-.-A-M-F-P-M",
            CoTType::AirMilFixedAsuw => "a-.-A-M-F-P-N",
            CoTType::AirMilFixedDroneRpvUav => "a-.-A-M-F-Q",
            CoTType::AirMilFixedReconn => "a-.-A-M-F-R",
            CoTType::AirMilFixedAew => "a-.-A-M-F-R-W",
            CoTType::AirMilFixedPhoto => "a-.-A-M-F-R-X",
            CoTType::AirMilFixedEsm => "a-.-A-M-F-R-Z",
            CoTType::AirMilFixedAswCarrierbased => "a-.-A-M-F-S",
            CoTType::AirMilFixedTrainer => "a-.-A-M-F-T",
            CoTType::AirMilFixedUtility => "a-.-A-M-F-U",
            CoTType::AirMilFixedUtilityHeavy => "a-.-A-M-F-U-H",
            CoTType::AirMilFixedUtilityLight => "a-.-A-M-F-U-L",
            CoTType::AirMilFixedUtilityMedium => "a-.-A-M-F-U-M",
            CoTType::AirMilFixedC3I => "a-.-A-M-F-Y",
            CoTType::AirMilRotor => "a-.-A-M-H",
            CoTType::AirMilRotorAttack => "a-.-A-M-H-A",
            CoTType::AirMilRotorTransport => "a-.-A-M-H-C",
            CoTType::AirMilRotorTransportHeavy => "a-.-A-M-H-C-H",
            CoTType::AirMilRotorTransportLight => "a-.-A-M-H-C-L",
            CoTType::AirMilRotorTransportMedium => "a-.-A-M-H-C-M",
            CoTType::AirMilRotorC2 => "a-.-A-M-H-D",
            CoTType::AirMilRotorCsar => "a-.-A-M-H-H",
            CoTType::AirMilRotorMinecountermeasures => "a-.-A-M-H-I",
            CoTType::AirMilRotorEcmJammer => "a-.-A-M-H-J",
            CoTType::AirMilRotorTanker => "a-.-A-M-H-K",
            CoTType::AirMilRotorSof => "a-.-A-M-H-M",
            CoTType::AirMilRotorMedevac => "a-.-A-M-H-O",
            CoTType::AirMilRotorDroneRpvUav => "a-.-A-M-H-Q",
            CoTType::AirMilRotorReconnaissance => "a-.-A-M-H-R",
            CoTType::AirMilRotorAswMpa => "a-.-A-M-H-S",
            CoTType::AirMilRotorTrainer => "a-.-A-M-H-T",
            CoTType::AirMilRotorUtility => "a-.-A-M-H-U",
            CoTType::AirMilRotorUtilityHeavy => "a-.-A-M-H-U-H",
            CoTType::AirMilRotorUtilityLight => "a-.-A-M-H-U-L",
            CoTType::AirMilRotorUtilityMedium => "a-.-A-M-H-U-M",
            CoTType::AirMilBlimp => "a-.-A-M-L",
            CoTType::AirWeapon => "a-.-A-W",
            CoTType::AirWeaponDecoy => "a-.-A-W-D",
            CoTType::AirWeaponMissileinflight => "a-.-A-W-M",
            CoTType::AirWeaponMissileAirlaunched => "a-.-A-W-M-A",
            CoTType::AirWeaponMissileAirtoair => "a-.-A-W-M-A-A",
            CoTType::AirWeaponMissileAirtosurface => "a-.-A-W-M-A-S",
            CoTType::AirWeaponMissileAttackmissile => "a-.-A-W-M-L",
            CoTType::AirWeaponMissileLandlaunchedmissile => "a-.-A-W-M-S",
            CoTType::AirWeaponMissileSam => "a-.-A-W-M-S-A",
            CoTType::AirWeaponMissileSamFixedsite => "a-.-A-W-M-S-A-f",
            CoTType::AirWeaponMissileSamManpad => "a-.-A-W-M-S-A-i",
            CoTType::AirWeaponMissileSamMobile => "a-.-A-W-M-S-A-m",
            CoTType::AirWeaponMissileSurfacetosurfaceSsm => "a-.-A-W-M-S-S",
            CoTType::AirWeaponMissileSubsurfacetosurfaceSSsm => "a-.-A-W-M-U",
            CoTType::Ground => "a-.-G",
            CoTType::GndEquipment => "a-.-G-E",
            CoTType::GndEquipSensor => "a-.-G-E-S",
            CoTType::GndEquipSensorEmplaced => "a-.-G-E-S-E",
            CoTType::GndEquipSensorRadar => "a-.-G-E-S-R",
            CoTType::GndEquipVehicle => "a-.-G-E-V",
            CoTType::GndEquipVehicArmorGun => "a-.-G-E-V-A",
            CoTType::GndEquipVehicArmorApc => "a-.-G-E-V-A-A",
            CoTType::GndEquipVehicArmorApcRecovery => "a-.-G-E-V-A-A-R",
            CoTType::GndEquipVehicArmorC2VAcv => "a-.-G-E-V-A-C",
            CoTType::GndEquipVehicArmorInfantry => "a-.-G-E-V-A-I",
            CoTType::GndEquipVehicArmorLight => "a-.-G-E-V-A-L",
            CoTType::GndEquipVehicArmorCombatservicesupport => "a-.-G-E-V-A-S",
            CoTType::GndEquipVehicArmorTank => "a-.-G-E-V-A-T",
            CoTType::GndEquipVehicArmorTankHeavy => "a-.-G-E-V-A-T-H",
            CoTType::GndEquipVehicArmorTankHeavyRecovery => "a-.-G-E-V-A-T-H-R",
            CoTType::GndEquipVehicArmorTankLight => "a-.-G-E-V-A-T-L",
            CoTType::GndEquipVehicArmorTankLightRecovery => "a-.-G-E-V-A-T-L-R",
            CoTType::GndEquipVehicArmorTankMedium => "a-.-G-E-V-A-T-M",
            CoTType::GndEquipVehicArmorTankMediumRecovery => "a-.-G-E-V-A-T-M-R",
            CoTType::GndEquipVehicCivilian => "a-.-G-E-V-C",
            CoTType::GndEquipVehicEngineer => "a-.-G-E-V-E",
            CoTType::GndEquipVehicMineclearingvehicle => "a-.-G-E-V-E-A",
            CoTType::GndEquipVehicArmoredmountedmineclearingvehicle => "a-.-G-E-V-E-A-A",
            CoTType::GndEquipVehicTrailermountedmineclearingvehicle => "a-.-G-E-V-E-A-T",
            CoTType::GndEquipVehicBridge => "a-.-G-E-V-E-B",
            CoTType::GndEquipVehicConstruction => "a-.-G-E-V-E-C",
            CoTType::GndEquipVehicDozer => "a-.-G-E-V-E-D",
            CoTType::GndEquipVehicEarthmover => "a-.-G-E-V-E-E",
            CoTType::GndEquipVehicMinelayingvehicle => "a-.-G-E-V-E-M",
            CoTType::GndEquipVehicTruckmountedwithvolcano => "a-.-G-E-V-E-M-L",
            CoTType::GndEquipVehicArmoredcarrierwithvolcano => "a-.-G-E-V-E-M-V",
            CoTType::GndEquipVehicLocomotive => "a-.-G-E-V-T",
            CoTType::GndEquipVehicUtility => "a-.-G-E-V-U",
            CoTType::GndEquipVehicBus => "a-.-G-E-V-U-B",
            CoTType::GndEquipVehicLimitedcrosscountrytruck => "a-.-G-E-V-U-L",
            CoTType::GndEquipVehicBoat => "a-.-G-E-V-U-R",
            CoTType::GndEquipVehicBoatLarge => "a-.-G-E-V-U-R-l",
            CoTType::GndEquipVehicBoatMed => "a-.-G-E-V-U-R-m",
            CoTType::GndEquipVehicBoatSmall => "a-.-G-E-V-U-R-s",
            CoTType::GndEquipVehicSemi => "a-.-G-E-V-U-S",
            CoTType::GndEquipVehicCrosscountrytruck => "a-.-G-E-V-U-X",
            CoTType::GndEquipVehicAmbulance => "a-.-G-E-V-m",
            CoTType::GndEquipWeapon => "a-.-G-E-W",
            CoTType::GndEquipWeaponAirdefensegun => "a-.-G-E-W-A",
            CoTType::GndEquipWeaponAaaHeavy => "a-.-G-E-W-A-H",
            CoTType::GndEquipWeaponAaaLight => "a-.-G-E-W-A-L",
            CoTType::GndEquipWeaponAaaMedium => "a-.-G-E-W-A-M",
            CoTType::GndEquipWeaponDirectfire => "a-.-G-E-W-D",
            CoTType::GndEquipWeaponDirectfireHeavy => "a-.-G-E-W-D-H",
            CoTType::GndEquipWeaponDirectfireHeavySelfpropelled => "a-.-G-E-W-D-H-S",
            CoTType::GndEquipWeaponDirectfireLight => "a-.-G-E-W-D-L",
            CoTType::GndEquipWeaponDirectfireLightSelfpropelled => "a-.-G-E-W-D-L-S",
            CoTType::GndEquipWeaponDirectfireMedium => "a-.-G-E-W-D-M",
            CoTType::GndEquipWeaponDirectfireMediumSelfpropelled => "a-.-G-E-W-D-M-S",
            CoTType::GndEquipWeaponAnittankgun => "a-.-G-E-W-G",
            CoTType::GndEquipWeaponAtgHeavy => "a-.-G-E-W-G-H",
            CoTType::GndEquipWeaponAtgLight => "a-.-G-E-W-G-L",
            CoTType::GndEquipWeaponAtgMedium => "a-.-G-E-W-G-M",
            CoTType::GndEquipWeaponAtgRecoilless => "a-.-G-E-W-G-R",
            CoTType::GndEquipWeaponHowitzer => "a-.-G-E-W-H",
            CoTType::GndEquipWeaponHowitzerHeavy => "a-.-G-E-W-H-H",
            CoTType::GndEquipWeaponHowitzerHeavySelfpropelled => "a-.-G-E-W-H-H-S",
            CoTType::GndEquipWeaponHowitzerLight => "a-.-G-E-W-H-L",
            CoTType::GndEquipWeaponHowitzerLightSelfpropelled => "a-.-G-E-W-H-L-S",
            CoTType::GndEquipWeaponHowitzerMedium => "a-.-G-E-W-H-M",
            CoTType::GndEquipWeaponHowitzerMediumSelfpropelled => "a-.-G-E-W-H-M-S",
            CoTType::GndEquipWeaponMissilelauncher => "a-.-G-E-W-M",
            CoTType::GndEquipWeaponMissilelauncherAdAirdefense => "a-.-G-E-W-M-A",
            CoTType::GndEquipWeaponMissilelauncherAdIntermediaterange => "a-.-G-E-W-M-A-I",
            CoTType::GndEquipWeaponMissilelauncherAdLongrange => "a-.-G-E-W-M-A-L",
            CoTType::GndEquipWeaponMissilelauncherAdShortrange => "a-.-G-E-W-M-A-S",
            CoTType::GndEquipWeaponMissilelauncherAdTheater => "a-.-G-E-W-M-A-T",
            CoTType::GndEquipWeaponMissilelauncherSsSurfacesurfSs => "a-.-G-E-W-M-S",
            CoTType::GndEquipWeaponMissilelauncherSsIntermediaterange => "a-.-G-E-W-M-S-I",
            CoTType::GndEquipWeaponMissilelauncherSsLongrange => "a-.-G-E-W-M-S-L",
            CoTType::GndEquipWeaponMissilelauncherSsShortrange => "a-.-G-E-W-M-S-S",
            CoTType::GndEquipWeaponMissilelauncherAtAntitank => "a-.-G-E-W-M-T",
            CoTType::GndEquipWeaponMissilelauncherAtHeavy => "a-.-G-E-W-M-T-H",
            CoTType::GndEquipWeaponMissilelauncherAtLight => "a-.-G-E-W-M-T-L",
            CoTType::GndEquipWeaponMissilelauncherAtMedium => "a-.-G-E-W-M-T-M",
            CoTType::GndEquipWeaponMortar => "a-.-G-E-W-O",
            CoTType::GndEquipWeaponMortarHeavy => "a-.-G-E-W-O-H",
            CoTType::GndEquipWeaponMortarLight => "a-.-G-E-W-O-L",
            CoTType::GndEquipWeaponMortarMedium => "a-.-G-E-W-O-M",
            CoTType::GndEquipWeaponRifleAutomaticweapon => "a-.-G-E-W-R",
            CoTType::GndEquipWeaponRifleHeavymachinegun => "a-.-G-E-W-R-H",
            CoTType::GndEquipWeaponRifleLightmachinegun => "a-.-G-E-W-R-L",
            CoTType::GndEquipWeaponRifleRifle => "a-.-G-E-W-R-R",
            CoTType::GndEquipWeaponRocketlauncherSingle => "a-.-G-E-W-S",
            CoTType::GndEquipWeaponRocketlauncherSingleHeavy => "a-.-G-E-W-S-H",
            CoTType::GndEquipWeaponRocketlauncherSingleLight => "a-.-G-E-W-S-L",
            CoTType::GndEquipWeaponRocketlauncherSingleMedium => "a-.-G-E-W-S-M",
            CoTType::GndEquipWeaponRocketlauncherAntitank => "a-.-G-E-W-T",
            CoTType::GndEquipWeaponRocketlauncherAtHeavy => "a-.-G-E-W-T-H",
            CoTType::GndEquipWeaponRocketlauncherAtLight => "a-.-G-E-W-T-L",
            CoTType::GndEquipWeaponRocketlauncherAtMedium => "a-.-G-E-W-T-M",
            CoTType::GndEquipWeaponRocketlauncherMultiple => "a-.-G-E-W-X",
            CoTType::GndEquipWeaponRocketlauncherMultipleHeavy => "a-.-G-E-W-X-H",
            CoTType::GndEquipWeaponRocketlauncherMultipleLight => "a-.-G-E-W-X-L",
            CoTType::GndEquipWeaponRocketlauncherMultipleMedium => "a-.-G-E-W-X-M",
            CoTType::GndEquipWeaponGrenadelauncher => "a-.-G-E-W-Z",
            CoTType::GndEquipWeaponGrenadelauncherHeavy => "a-.-G-E-W-Z-H",
            CoTType::GndEquipWeaponGrenadelauncherLight => "a-.-G-E-W-Z-L",
            CoTType::GndEquipWeaponGrenadelauncherMedium => "a-.-G-E-W-Z-M",
            CoTType::GndEquipSpecialequipment => "a-.-G-E-X",
            CoTType::GndEquipFlamethrower => "a-.-G-E-X-F",
            CoTType::GndEquipLaser => "a-.-G-E-X-L",
            CoTType::GndEquipLandmines => "a-.-G-E-X-M",
            CoTType::GndEquipClaymore => "a-.-G-E-X-M-C",
            CoTType::GndEquipLessthanlethal => "a-.-G-E-X-M-L",
            CoTType::GndEquipNbcequipment => "a-.-G-E-X-N",
            CoTType::GndBuilding => "a-.-G-I",
            CoTType::GndStructureBaseMilitary => "a-.-G-I-B",
            CoTType::GndStructureBaseAirbase => "a-.-G-I-B-A",
            CoTType::GndStructureBaseNavalbase => "a-.-G-I-B-N",
            CoTType::GndStructureCivilian => "a-.-G-I-c",
            CoTType::GndStructureCivilianBridge => "a-.-G-I-c-b",
            CoTType::GndStructureCivilianFenceorwallorbarrier => "a-.-G-I-c-bar",
            CoTType::GndStructureCivilianCanal => "a-.-G-I-c-can",
            CoTType::GndStructureCivilianCanalLock => "a-.-G-I-c-can-l",
            CoTType::GndStructureCivilianFarm => "a-.-G-I-c-frm",
            CoTType::GndStructureCivilianLevee => "a-.-G-I-c-l",
            CoTType::GndStructureCivilianMonument => "a-.-G-I-c-mon",
            CoTType::GndStructureCivilianOffices => "a-.-G-I-c-o",
            CoTType::GndStructureCivilianResidenceorapartmentorhotel => "a-.-G-I-c-rah",
            CoTType::GndStructureCivilianReligous => "a-.-G-I-c-rel",
            CoTType::GndStructureCivilianResevoir => "a-.-G-I-c-res",
            CoTType::GndStructureCivilianRetail => "a-.-G-I-c-ret",
            CoTType::GndStructureCivilianSchool => "a-.-G-I-c-sch",
            CoTType::GndStructureCivilianVip => "a-.-G-I-c-vip",
            CoTType::GndStructureCivilianWarehouse => "a-.-G-I-c-whs",
            CoTType::GndStructureFactory => "a-.-G-I-E",
            CoTType::GndStructureFactoryHeavy => "a-.-G-I-E-h",
            CoTType::GndStructureFactoryLight => "a-.-G-I-E-l",
            CoTType::GndStructureFactoryPlantother => "a-.-G-I-E-o",
            CoTType::GndStructureGovernment => "a-.-G-I-G",
            CoTType::GndStructureImfacilities => "a-.-G-I-i",
            CoTType::GndStructureImfacilitiesEmergencymanagement => "a-.-G-I-i-e",
            CoTType::GndStructureImfacilitiesEmergencymanagementEmergencyoperationscenter => {
                "a-.-G-I-i-e-eoc"
            }
            CoTType::GndStructureImfacilitiesEmergencymanagementIncidentcommander => {
                "a-.-G-I-i-e-ic"
            }
            CoTType::GndStructureImfacilitiesEmergencymanagementIncidentcommunicationscenter => {
                "a-.-G-I-i-e-icc"
            }
            CoTType::GndStructureImfacilitiesEmergencymanagementIncidentcommandpost => {
                "a-.-G-I-i-e-icp"
            }
            CoTType::GndStructureImfacilitiesEmergencymanagementWarroom => "a-.-G-I-i-e-wr",
            CoTType::GndStructureImfacilitiesFireandhazmat => "a-.-G-I-i-f",
            CoTType::GndStructureImfacilitiesFireandhazmatFiredepartment => "a-.-G-I-i-f-fd",
            CoTType::GndStructureImfacilitiesHealthandmedical => "a-.-G-I-i-h",
            CoTType::GndStructureImfacilitiesHealthandmedicalMedicaldispatch => "a-.-G-I-i-h-md",
            CoTType::GndStructureImfacilitiesLawenforcement => "a-.-G-I-i-l",
            CoTType::GndStructureImfacilitiesLawenforcementPolicecheckpoint => "a-.-G-I-i-l-cp",
            CoTType::GndStructureImfacilitiesLawenforcementHighwaypatrol => "a-.-G-I-i-l-hp",
            CoTType::GndStructureImfacilitiesLawenforcementLocalpolice => "a-.-G-I-i-l-lp",
            CoTType::GndStructureImfacilitiesLawenforcementNotrespassingzone => "a-.-G-I-i-l-ntz",
            CoTType::GndStructureImfacilitiesLawenforcementSheriffsdepartment => "a-.-G-I-i-l-sd",
            CoTType::GndStructureImfacilitiesLawenforcementStatepolice => "a-.-G-I-i-l-sp",
            CoTType::GndStructureImfacilitiesEmergencymedicalservices => "a-.-G-I-i-m",
            CoTType::GndStructureImfacilitiesOther => "a-.-G-I-i-o",
            CoTType::GndStructureImfacilitiesOtherAgencydispatch => "a-.-G-I-i-o-ad",
            CoTType::GndStructureImfacilitiesOtherAgencydispatchEms => "a-.-G-I-i-o-ad-ems",
            CoTType::GndStructureImfacilitiesOtherAgencydispatchFire => "a-.-G-I-i-o-ad-f",
            CoTType::GndStructureImfacilitiesOtherAgencydispatchHighway => "a-.-G-I-i-o-ad-hwy",
            CoTType::GndStructureImfacilitiesOtherAgencydispatchMedia => "a-.-G-I-i-o-ad-mda",
            CoTType::GndStructureImfacilitiesOtherAgencydispatchPolicelocal => "a-.-G-I-i-o-ad-pl",
            CoTType::GndStructureImfacilitiesOtherAgencydispatchPolicestation => {
                "a-.-G-I-i-o-ad-ps"
            }
            CoTType::GndStructureImfacilitiesOtherAgencydispatchTow => "a-.-G-I-i-o-ad-tow",
            CoTType::GndStructureImfacilitiesOtherAgencydispatchTransit => "a-.-G-I-i-o-ad-ts",
            CoTType::GndStructureImfacilitiesOtherBordercontrolcheckpoint => "a-.-G-I-i-o-bcc",
            CoTType::GndStructureImfacilitiesOtherDmz => "a-.-G-I-i-o-dmz",
            CoTType::GndStructureImfacilitiesOtherIndependentserviceprovider => "a-.-G-I-i-o-isp",
            CoTType::GndStructureImfacilitiesOtherMobilecenter => "a-.-G-I-i-o-mc",
            CoTType::GndStructureImfacilitiesOtherMobilecenterFire => "a-.-G-I-i-o-mc-f",
            CoTType::GndStructureImfacilitiesOtherMobilecenterMedical => "a-.-G-I-i-o-mc-h",
            CoTType::GndStructureImfacilitiesOtherMobilecenterLawenforcement => "a-.-G-I-i-o-mc-l",
            CoTType::GndStructureImfacilitiesOtherMessagecenter => "a-.-G-I-i-o-mes",
            CoTType::GndStructureImfacilitiesOtherMobilizationcenter => "a-.-G-I-i-o-mob",
            CoTType::GndStructureImfacilitiesOtherMobilizationcenterEms => "a-.-G-I-i-o-mob-ems",
            CoTType::GndStructureImfacilitiesOtherMobilizationcenterTransit => "a-.-G-I-i-o-mob-es",
            CoTType::GndStructureImfacilitiesOtherMobilizationcenterFire => "a-.-G-I-i-o-mob-f",
            CoTType::GndStructureImfacilitiesOtherMobilizationcenterHighway => {
                "a-.-G-I-i-o-mob-hwy"
            }
            CoTType::GndStructureImfacilitiesOtherMobilizationcenterMedia => "a-.-G-I-i-o-mob-mda",
            CoTType::GndStructureImfacilitiesOtherMobilizationcenterPolicelocal => {
                "a-.-G-I-i-o-mob-pl"
            }
            CoTType::GndStructureImfacilitiesOtherMobilizationcenterPolicestation => {
                "a-.-G-I-i-o-mob-ps"
            }
            CoTType::GndStructureImfacilitiesOtherMobilizationcenterTow => "a-.-G-I-i-o-mob-tow",
            CoTType::GndStructureImfacilitiesOtherPublicsafety => "a-.-G-I-i-o-ps",
            CoTType::GndStructureImfacilitiesOtherReportinglocations => "a-.-G-I-i-o-rl",
            CoTType::GndStructureImfacilitiesOtherStagingarea => "a-.-G-I-i-o-sa",
            CoTType::GndStructureImfacilitiesOtherStagingareaEms => "a-.-G-I-i-o-sa-ems",
            CoTType::GndStructureImfacilitiesOtherStagingareaFire => "a-.-G-I-i-o-sa-f",
            CoTType::GndStructureImfacilitiesOtherStagingareaHighway => "a-.-G-I-i-o-sa-hwy",
            CoTType::GndStructureImfacilitiesOtherStagingareaMedia => "a-.-G-I-i-o-sa-mda",
            CoTType::GndStructureImfacilitiesOtherStagingareaPolicelocal => "a-.-G-I-i-o-sa-pl",
            CoTType::GndStructureImfacilitiesOtherStagingareaPolicestation => "a-.-G-I-i-o-sa-ps",
            CoTType::GndStructureImfacilitiesOtherStagingareaTow => "a-.-G-I-i-o-sa-tow",
            CoTType::GndStructureImfacilitiesOtherStagingareaTransit => "a-.-G-I-i-o-sa-ts",
            CoTType::GndStructureImfacilitiesOtherSecuritycheckpoint => "a-.-G-I-i-o-sc",
            CoTType::GndStructureImfacilitiesOtherSegment => "a-.-G-I-i-o-seg",
            CoTType::GndStructureImfacilitiesOtherTransportation => "a-.-G-I-i-o-t",
            CoTType::GndStructureImfacilitiesOtherTransportationTransitincidentmanagementcenter => {
                "a-.-G-I-i-o-t-im"
            }
            CoTType::GndStructureImfacilitiesOtherTransportationTransitmanagementcenter => {
                "a-.-G-I-i-o-t-tc"
            }
            CoTType::GndStructureImfacilitiesOtherTransportationTrafficincservicepatroldispatch => {
                "a-.-G-I-i-o-t-td"
            }
            CoTType::GndStructureImfacilitiesOtherTransportationTrafficmanagementcenter => {
                "a-.-G-I-i-o-t-tm"
            }
            CoTType::GndStructureImfacilitiesOtherTemp => "a-.-G-I-i-o-tmp",
            CoTType::GndStructureImfacilitiesOtherTempFire => "a-.-G-I-i-o-tmp-f",
            CoTType::GndStructureImfacilitiesOtherTempMedical => "a-.-G-I-i-o-tmp-h",
            CoTType::GndStructureImfacilitiesOtherTempLawenforcement => "a-.-G-I-i-o-tmp-l",
            CoTType::GndStructureImfacilitiesPublicworks => "a-.-G-I-i-p",
            CoTType::GndStructureMilMilitarymaterielfacility => "a-.-G-I-M",
            CoTType::GndStructureMilAircraftproduction => "a-.-G-I-M-A",
            CoTType::GndStructureMilNbcwarfareproduction => "a-.-G-I-M-C",
            CoTType::GndStructureMilAmmunitionproduction => "a-.-G-I-M-E",
            CoTType::GndStructureMilNuclearenergy => "a-.-G-I-M-F",
            CoTType::GndStructureMilAtomicenergyreactor => "a-.-G-I-M-F-A",
            CoTType::GndStructureMilNuclearmaterialproduction => "a-.-G-I-M-F-P",
            CoTType::GndStructureMilNuclearWeaponsgrade => "a-.-G-I-M-F-P-W",
            CoTType::GndStructureMilNuclearmaterialstorage => "a-.-G-I-M-F-S",
            CoTType::GndStructureMilArmamentproduction => "a-.-G-I-M-G",
            CoTType::GndStructureMilMissileSpacesystemproduction => "a-.-G-I-M-M",
            CoTType::GndStructureMilEngineeringequipmentproduction => "a-.-G-I-M-N",
            CoTType::GndStructureMilBridge => "a-.-G-I-M-N-B",
            CoTType::GndStructureMilBridgeLarge => "a-.-G-I-M-N-B-l",
            CoTType::GndStructureMilBridgeMed => "a-.-G-I-M-N-B-m",
            CoTType::GndStructureMilBridgeSmall => "a-.-G-I-M-N-B-s",
            CoTType::GndStructureMilCanal => "a-.-G-I-M-N-c",
            CoTType::GndStructureMilShipconstruction => "a-.-G-I-M-S",
            CoTType::GndStructureMilMilitaryvehicleproduction => "a-.-G-I-M-V",
            CoTType::GndStructureProcessingfacility => "a-.-G-I-P",
            CoTType::GndStructureDecon => "a-.-G-I-P-D",
            CoTType::GndStructureRawmaterialproductionStorage => "a-.-G-I-R",
            CoTType::GndStructureMine => "a-.-G-I-R-M",
            CoTType::GndStructureNbc => "a-.-G-I-R-N",
            CoTType::GndStructureBiological => "a-.-G-I-R-N-B",
            CoTType::GndStructureChemical => "a-.-G-I-R-N-C",
            CoTType::GndStructureNuclear => "a-.-G-I-R-N-N",
            CoTType::GndStructurePetroleumGasOil => "a-.-G-I-R-P",
            CoTType::GndStructurePetroleumGasOilRefinery => "a-.-G-I-R-P-r",
            CoTType::GndStructureTransportfacility => "a-.-G-I-T",
            CoTType::GndStructureTransportfacilityAirport => "a-.-G-I-T-a",
            CoTType::GndStructureTransportfacilityHelibase => "a-.-G-I-T-hb",
            CoTType::GndStructureTransportfacilityHelispot => "a-.-G-I-T-hs",
            CoTType::GndStructureTransportfacilityLandport => "a-.-G-I-T-l",
            CoTType::GndStructureTransportfacilityParkinggarage => "a-.-G-I-T-pg",
            CoTType::GndStructureTransportfacilityRailroad => "a-.-G-I-T-r",
            CoTType::GndStructureTransportfacilitySeaport => "a-.-G-I-T-s",
            CoTType::GndStructureUtilityServiceResearchUtilityfacility => "a-.-G-I-U",
            CoTType::GndStructureUtilityElectricpower => "a-.-G-I-U-E",
            CoTType::GndStructureUtilityElectricpowerCoalpowerplant => "a-.-G-I-U-E-c",
            CoTType::GndStructureUtilityElectricpowerHydroelectricpowerplant => "a-.-G-I-U-E-h",
            CoTType::GndStructureUtilityElectricpowerOtherpower => "a-.-G-I-U-E-o",
            CoTType::GndStructureUtilityElectricpowerPowersubstation => "a-.-G-I-U-E-ps",
            CoTType::GndStructureUtilityDam => "a-.-G-I-U-E-D",
            CoTType::GndStructureUtilityFossilfuel => "a-.-G-I-U-E-F",
            CoTType::GndStructureUtilityNuclearplant => "a-.-G-I-U-E-N",
            CoTType::GndStructureUtilityPublicwaterservices => "a-.-G-I-U-P",
            CoTType::GndStructureUtilityTechnologicalresearch => "a-.-G-I-U-R",
            CoTType::GndStructureUtilityTelecommunications => "a-.-G-I-U-T",
            CoTType::GndStructureUtilityTelecommunicationsCommunications => "a-.-G-I-U-T-com",
            CoTType::GndStructureUtilityTelecommunicationsCommunicationsAntennafarm => {
                "a-.-G-I-U-T-com-af"
            }
            CoTType::GndStructureUtilityTelecommunicationsCommunicationsSatellitecommunications => {
                "a-.-G-I-U-T-com-sat"
            }
            CoTType::GndStructureUtilityTelecommunicationsCommunicationsTower => {
                "a-.-G-I-U-T-com-tow"
            }
            CoTType::GndStructureUtilityTelecommunicationsRadio => "a-.-G-I-U-T-r",
            CoTType::GndStructureUtilityTelecommunicationsRadioStation => "a-.-G-I-U-T-r-s",
            CoTType::GndStructureUtilityTelecommunicationsTelephone => "a-.-G-I-U-T-tp",
            CoTType::GndStructureUtilityTelecommunicationsTelephoneTelephoneswitching => {
                "a-.-G-I-U-T-tp-ts"
            }
            CoTType::GndStructureUtilityTelecommunicationsTelevision => "a-.-G-I-U-T-tv",
            CoTType::GndStructureUtilityTelecommunicationsTelevisionCable => "a-.-G-I-U-T-tv-c",
            CoTType::GndStructureUtilityTelecommunicationsTelevisionStation => "a-.-G-I-U-T-tv-s",
            CoTType::GndStructureMedicalfacility => "a-.-G-I-X",
            CoTType::GndStructureHospital => "a-.-G-I-X-H",
            CoTType::GndStructureMedicalfacilityHospitalHospitalcarefacility => "a-.-G-I-X-hcf",
            CoTType::GndStructureRoad => "a-.-G-I-r",
            CoTType::GndStructureRoadHighway => "a-.-G-I-r-h",
            CoTType::GndStructureRoadImproved => "a-.-G-I-r-i",
            CoTType::GndStructureRoadRestarea => "a-.-G-I-r-ra",
            CoTType::GndStructureRoadUnimproved => "a-.-G-I-r-u",
            CoTType::GndUnit => "a-.-G-U",
            CoTType::GndCombat => "a-.-G-U-C",
            CoTType::GndCombatArmor => "a-.-G-U-C-A",
            CoTType::GndCombatArmorAntiarmor => "a-.-G-U-C-A-A",
            CoTType::GndCombatArmorAntiarmorarmored => "a-.-G-U-C-A-A-A",
            CoTType::GndCombatArmorAntiarmorarmoredairassault => "a-.-G-U-C-A-A-A-S",
            CoTType::GndCombatArmorAntiarmorarmoredtracked => "a-.-G-U-C-A-A-A-T",
            CoTType::GndCombatArmorAntiarmorarmoredwheeled => "a-.-G-U-C-A-A-A-W",
            CoTType::GndCombatArmorAntiarmorarctic => "a-.-G-U-C-A-A-C",
            CoTType::GndCombatArmorAntiarmordismounted => "a-.-G-U-C-A-A-D",
            CoTType::GndCombatArmorAntiarmorlight => "a-.-G-U-C-A-A-L",
            CoTType::GndCombatArmorAntiarmorairborne => "a-.-G-U-C-A-A-M",
            CoTType::GndCombatArmorAntiarmormotorized => "a-.-G-U-C-A-A-O",
            CoTType::GndCombatArmorAntiarmormotorizedairassault => "a-.-G-U-C-A-A-O-S",
            CoTType::GndCombatArmorAntiarmorairassault => "a-.-G-U-C-A-A-S",
            CoTType::GndCombatArmorAntiarmormountain => "a-.-G-U-C-A-A-U",
            CoTType::GndCombatArmorArmortrack => "a-.-G-U-C-A-T",
            CoTType::GndCombatArmorArmortrackairborne => "a-.-G-U-C-A-T-A",
            CoTType::GndCombatArmorArmortrackHeavy => "a-.-G-U-C-A-T-H",
            CoTType::GndCombatArmorArmortrackLight => "a-.-G-U-C-A-T-L",
            CoTType::GndCombatArmorArmortrackMedium => "a-.-G-U-C-A-T-M",
            CoTType::GndCombatArmorArmortrackRecovery => "a-.-G-U-C-A-T-R",
            CoTType::GndCombatArmorArmortrackamphibious => "a-.-G-U-C-A-T-W",
            CoTType::GndCombatArmorArmortrackamphibiousrecovery => "a-.-G-U-C-A-T-W-R",
            CoTType::GndCombatArmorArmorWheeled => "a-.-G-U-C-A-W",
            CoTType::GndCombatArmorArmorWheeledairborne => "a-.-G-U-C-A-W-A",
            CoTType::GndCombatArmorArmorWheeledheavy => "a-.-G-U-C-A-W-H",
            CoTType::GndCombatArmorArmorWheeledlight => "a-.-G-U-C-A-W-L",
            CoTType::GndCombatArmorArmorWheeledmedium => "a-.-G-U-C-A-W-M",
            CoTType::GndCombatArmorArmorWheeledrecovery => "a-.-G-U-C-A-W-R",
            CoTType::GndCombatArmorArmorWheeledairassault => "a-.-G-U-C-A-W-S",
            CoTType::GndCombatArmorArmorWheeledamphibious => "a-.-G-U-C-A-W-W",
            CoTType::GndCombatArmorArmorWheeledamphibiousrecovery => "a-.-G-U-C-A-W-W-R",
            CoTType::GndCombatDefenseAirdefense => "a-.-G-U-C-D",
            CoTType::GndCombatDefenseComposite => "a-.-G-U-C-D-C",
            CoTType::GndCombatDefenseGununit => "a-.-G-U-C-D-G",
            CoTType::GndCombatDefenseHMad => "a-.-G-U-C-D-H",
            CoTType::GndCombatDefenseHawk => "a-.-G-U-C-D-H-H",
            CoTType::GndCombatDefensePatriot => "a-.-G-U-C-D-H-P",
            CoTType::GndCombatDefenseAirdefensemissile => "a-.-G-U-C-D-M",
            CoTType::GndCombatDefenseAirdefensemissileheavy => "a-.-G-U-C-D-M-H",
            CoTType::GndCombatDefenseAirdefensemissilelight => "a-.-G-U-C-D-M-L",
            CoTType::GndCombatDefenseAirdefensemissilemotorizedAvenger => "a-.-G-U-C-D-M-L-A",
            CoTType::GndCombatDefenseAirdefensemissilemedium => "a-.-G-U-C-D-M-M",
            CoTType::GndCombatDefenseTheatermissiledefenseunit => "a-.-G-U-C-D-O",
            CoTType::GndCombatDefenseShortrange => "a-.-G-U-C-D-S",
            CoTType::GndCombatDefenseStinger => "a-.-G-U-C-D-S-S",
            CoTType::GndCombatDefenseVulcan => "a-.-G-U-C-D-S-V",
            CoTType::GndCombatDefenseTargetingunit => "a-.-G-U-C-D-T",
            CoTType::GndCombatEngineerEngineer => "a-.-G-U-C-E",
            CoTType::GndCombatEngineerEngineercombat => "a-.-G-U-C-E-C",
            CoTType::GndCombatEngineerEngineercombatairborne => "a-.-G-U-C-E-C-A",
            CoTType::GndCombatEngineerEngineercombatarctic => "a-.-G-U-C-E-C-C",
            CoTType::GndCombatEngineerEngineercombatheavy => "a-.-G-U-C-E-C-H",
            CoTType::GndCombatEngineerEngineercombatlightSapper => "a-.-G-U-C-E-C-L",
            CoTType::GndCombatEngineerEngineercombatmedium => "a-.-G-U-C-E-C-M",
            CoTType::GndCombatEngineerEngineercombatmountain => "a-.-G-U-C-E-C-O",
            CoTType::GndCombatEngineerEngineercombatrecon => "a-.-G-U-C-E-C-R",
            CoTType::GndCombatEngineerEngineercombatairassault => "a-.-G-U-C-E-C-S",
            CoTType::GndCombatEngineerEngineercombatmechanizedTrack => "a-.-G-U-C-E-C-T",
            CoTType::GndCombatEngineerEngineercombatmotorized => "a-.-G-U-C-E-C-W",
            CoTType::GndCombatEngineerEngineerconstruction => "a-.-G-U-C-E-N",
            CoTType::GndCombatEngineerEngineernavalconstruction => "a-.-G-U-C-E-N-N",
            CoTType::GndCombatArtilleryFixed => "a-.-G-U-C-F",
            CoTType::GndCombatHowitzerGun => "a-.-G-U-C-F-H",
            CoTType::GndCombatAirborne => "a-.-G-U-C-F-H-A",
            CoTType::GndCombatArctic => "a-.-G-U-C-F-H-C",
            CoTType::GndCombatArtilleryMobile => "a-.-G-U-C-F-H-E",
            CoTType::GndCombatHeavy => "a-.-G-U-C-F-H-H",
            CoTType::GndCombatLight => "a-.-G-U-C-F-H-L",
            CoTType::GndCombatMedium => "a-.-G-U-C-F-H-M",
            CoTType::GndCombatMountain => "a-.-G-U-C-F-H-O",
            CoTType::GndCombatAirassault => "a-.-G-U-C-F-H-S",
            CoTType::GndCombatAmphibious => "a-.-G-U-C-F-H-X",
            CoTType::GndCombatMortar => "a-.-G-U-C-F-M",
            CoTType::GndCombatAmphibiousmortar => "a-.-G-U-C-F-M-L",
            CoTType::GndCombatSelfpropelledSpTrackedmortar => "a-.-G-U-C-F-M-S",
            CoTType::GndCombatTowedmortar => "a-.-G-U-C-F-M-T",
            CoTType::GndCombatTowedairbornemortar => "a-.-G-U-C-F-M-T-A",
            CoTType::GndCombatTowedarcticmortar => "a-.-G-U-C-F-M-T-C",
            CoTType::GndCombatTowedmountainmortar => "a-.-G-U-C-F-M-T-O",
            CoTType::GndCombatTowedairassaultmortar => "a-.-G-U-C-F-M-T-S",
            CoTType::GndCombatSpwheeledmortar => "a-.-G-U-C-F-M-W",
            CoTType::GndCombatMeteorological => "a-.-G-U-C-F-O",
            CoTType::GndCombatAirbornemeteorological => "a-.-G-U-C-F-O-A",
            CoTType::GndCombatLightmeteorological => "a-.-G-U-C-F-O-L",
            CoTType::GndCombatMountainmeteorological => "a-.-G-U-C-F-O-O",
            CoTType::GndCombatAirassaultmeteorological => "a-.-G-U-C-F-O-S",
            CoTType::GndCombatRocket => "a-.-G-U-C-F-R",
            CoTType::GndCombatRocketsFixed => "a-.-G-U-C-F-R-M",
            CoTType::GndCombatRocketsMobile => "a-.-G-U-C-F-R-M-R",
            CoTType::GndCombatMultirocketselfpropelled => "a-.-G-U-C-F-R-M-S",
            CoTType::GndCombatMultirockettowed => "a-.-G-U-C-F-R-M-T",
            CoTType::GndCombatSinglerocketlauncher => "a-.-G-U-C-F-R-S",
            CoTType::GndCombatSinglerockettruck => "a-.-G-U-C-F-R-S-R",
            CoTType::GndCombatSinglerocketselfpropelled => "a-.-G-U-C-F-R-S-S",
            CoTType::GndCombatSinglerockettowed => "a-.-G-U-C-F-R-S-T",
            CoTType::GndCombatArtillerysurvey => "a-.-G-U-C-F-S",
            // CoTType::GndCombatAirborne => "a-.-G-U-C-F-S-A",
            // CoTType::GndCombatLight => "a-.-G-U-C-F-S-L",
            // CoTType::GndCombatMountain => "a-.-G-U-C-F-S-O",
            // CoTType::GndCombatAirassault => "a-.-G-U-C-F-S-S",
            CoTType::GndCombatTargetacquisition => "a-.-G-U-C-F-T",
            CoTType::GndCombatAnglico => "a-.-G-U-C-F-T-A",
            CoTType::GndCombatColtFist => "a-.-G-U-C-F-T-C",
            CoTType::GndCombatDismountedcoltFist => "a-.-G-U-C-F-T-C-D",
            CoTType::GndCombatTrackedcoltFist => "a-.-G-U-C-F-T-C-M",
            CoTType::GndCombatFlashOptical => "a-.-G-U-C-F-T-F",
            CoTType::GndCombatRadar => "a-.-G-U-C-F-T-R",
            CoTType::GndCombatSound => "a-.-G-U-C-F-T-S",
            CoTType::GndCombatInfantryTroopsOpen => "a-.-G-U-C-I",
            CoTType::GndCombatInfantryAirborne => "a-.-G-U-C-I-A",
            CoTType::GndCombatInfantryArctic => "a-.-G-U-C-I-C",
            CoTType::GndCombatInfantryFightingvehicle => "a-.-G-U-C-I-I",
            CoTType::GndCombatInfantryLight => "a-.-G-U-C-I-L",
            CoTType::GndCombatInfantryMotorized => "a-.-G-U-C-I-M",
            CoTType::GndCombatInfantryNaval => "a-.-G-U-C-I-N",
            CoTType::GndCombatInfantryMountain => "a-.-G-U-C-I-O",
            CoTType::GndCombatInfantryAirassault => "a-.-G-U-C-I-S",
            CoTType::GndCombatInfantryMechanized => "a-.-G-U-C-I-Z",
            CoTType::GndCombatInfantryDugin => "a-.-G-U-C-I-d",
            CoTType::GndCombatMissileSurfsurf => "a-.-G-U-C-M",
            CoTType::GndCombatMissileSurfsurfStrategic => "a-.-G-U-C-M-S",
            CoTType::GndCombatMissileSurfsurfTactical => "a-.-G-U-C-M-T",
            CoTType::GndCombatRecon => "a-.-G-U-C-R",
            CoTType::GndCombatReconAirborne => "a-.-G-U-C-R-A",
            CoTType::GndCombatReconArctic => "a-.-G-U-C-R-C",
            CoTType::GndCombatReconHorse => "a-.-G-U-C-R-H",
            CoTType::GndCombatReconLight => "a-.-G-U-C-R-L",
            CoTType::GndCombatReconMountain => "a-.-G-U-C-R-O",
            CoTType::GndCombatReconMarine => "a-.-G-U-C-R-R",
            CoTType::GndCombatReconMarinedivision => "a-.-G-U-C-R-R-D",
            CoTType::GndCombatReconMarineforce => "a-.-G-U-C-R-R-F",
            CoTType::GndCombatReconMarinelightarmored => "a-.-G-U-C-R-R-L",
            CoTType::GndCombatReconAirassault => "a-.-G-U-C-R-S",
            CoTType::GndCombatReconCavalry => "a-.-G-U-C-R-V",
            CoTType::GndCombatReconCavalryarmored => "a-.-G-U-C-R-V-A",
            CoTType::GndCombatReconCavalryground => "a-.-G-U-C-R-V-G",
            CoTType::GndCombatReconCavalrymotorized => "a-.-G-U-C-R-V-M",
            CoTType::GndCombatReconCavalryair => "a-.-G-U-C-R-V-O",
            CoTType::GndCombatReconLongrangesurveillance => "a-.-G-U-C-R-X",
            CoTType::GndCombatSecurityInternalsecurityforces => "a-.-G-U-C-S",
            CoTType::GndCombatSecurityAviation => "a-.-G-U-C-S-A",
            CoTType::GndCombatSecurityGround => "a-.-G-U-C-S-G",
            CoTType::GndCombatSecurityMechanizedground => "a-.-G-U-C-S-G-A",
            CoTType::GndCombatSecurityDismountedground => "a-.-G-U-C-S-G-D",
            CoTType::GndCombatSecurityMotorizedground => "a-.-G-U-C-S-G-M",
            CoTType::GndCombatSecurityWheeledmechanized => "a-.-G-U-C-S-M",
            CoTType::GndCombatSecurityRail => "a-.-G-U-C-S-R",
            CoTType::GndCombatSecurityRiverine => "a-.-G-U-C-S-W",
            CoTType::GndCombatAviationAviation => "a-.-G-U-C-V",
            CoTType::GndCombatAviationComposite => "a-.-G-U-C-V-C",
            CoTType::GndCombatAviationFixedwing => "a-.-G-U-C-V-F",
            CoTType::GndCombatAviationAttackfixedwing => "a-.-G-U-C-V-F-A",
            CoTType::GndCombatAviationReconfixedwing => "a-.-G-U-C-V-F-R",
            CoTType::GndCombatAviationUtilityfixedwing => "a-.-G-U-C-V-F-U",
            CoTType::GndCombatAviationRotarywing => "a-.-G-U-C-V-R",
            CoTType::GndCombatAviationAttackrotarywing => "a-.-G-U-C-V-R-A",
            CoTType::GndCombatAviationMinecountermeasurerotarywing => "a-.-G-U-C-V-R-M",
            CoTType::GndCombatAviationScoutrotarywing => "a-.-G-U-C-V-R-S",
            CoTType::GndCombatAviationUtilityrotarywing => "a-.-G-U-C-V-R-U",
            CoTType::GndCombatAviationC2Rotarywing => "a-.-G-U-C-V-R-U-C",
            CoTType::GndCombatAviationMedevacrotarywing => "a-.-G-U-C-V-R-U-E",
            CoTType::GndCombatAviationHeavyutilityrotarywing => "a-.-G-U-C-V-R-U-H",
            CoTType::GndCombatAviationLightutilityrotarywing => "a-.-G-U-C-V-R-U-L",
            CoTType::GndCombatAviationMediumutilityrotarywing => "a-.-G-U-C-V-R-U-M",
            CoTType::GndCombatAviationAntisubmarinewarfarerotarywing => "a-.-G-U-C-V-R-W",
            CoTType::GndCombatAviationHeloDual => "a-.-G-U-C-V-R-d",
            CoTType::GndCombatAviationHeloSingle => "a-.-G-U-C-V-R-s",
            CoTType::GndCombatAviationSearchandrescue => "a-.-G-U-C-V-S",
            CoTType::GndCombatAviationUnmannedaerialvehicle => "a-.-G-U-C-V-U",
            CoTType::GndCombatAviationUnmannedaerialvehiclefixedwing => "a-.-G-U-C-V-U-F",
            CoTType::GndCombatAviationUnmannedaerialvehiclerotarywing => "a-.-G-U-C-V-U-R",
            CoTType::GndCombatAviationVerticalShorttakeoffandlanding => "a-.-G-U-C-V-V",
            CoTType::GndSpecialc2Headquarterscomponent => "a-.-G-U-H",
            CoTType::GndCombatservicesupport => "a-.-G-U-S",
            CoTType::GndAdministrativeAdmin => "a-.-G-U-S-A",
            CoTType::GndAdmincorps => "a-.-G-U-S-A-C",
            CoTType::GndFinance => "a-.-G-U-S-A-F",
            CoTType::GndFinancecorps => "a-.-G-U-S-A-F-C",
            CoTType::GndFinancetheater => "a-.-G-U-S-A-F-T",
            CoTType::GndJudgeadvocategeneralJag => "a-.-G-U-S-A-J",
            CoTType::GndJagcorps => "a-.-G-U-S-A-J-C",
            CoTType::GndJagtheater => "a-.-G-U-S-A-J-T",
            CoTType::GndLabor => "a-.-G-U-S-A-L",
            CoTType::GndLaborcorps => "a-.-G-U-S-A-L-C",
            CoTType::GndLabortheater => "a-.-G-U-S-A-L-T",
            CoTType::GndMortuaryGravesregistry => "a-.-G-U-S-A-M",
            CoTType::GndMortuaryGravesregistrycorps => "a-.-G-U-S-A-M-C",
            CoTType::GndMortuaryGravesregistrytheater => "a-.-G-U-S-A-M-T",
            CoTType::GndPostal => "a-.-G-U-S-A-O",
            CoTType::GndPostalcorps => "a-.-G-U-S-A-O-C",
            CoTType::GndPostaltheater => "a-.-G-U-S-A-O-T",
            CoTType::GndPublicaffairs => "a-.-G-U-S-A-P",
            CoTType::GndPublicaffairsbroadcast => "a-.-G-U-S-A-P-B",
            CoTType::GndPublicaffairsbroadcastcorps => "a-.-G-U-S-A-P-B-C",
            CoTType::GndPublicaffairsbroadcasttheater => "a-.-G-U-S-A-P-B-T",
            CoTType::GndPublicaffairscorps => "a-.-G-U-S-A-P-C",
            CoTType::GndPublicaffairsjointinformationbureau => "a-.-G-U-S-A-P-M",
            CoTType::GndPublicaffairsjibcorps => "a-.-G-U-S-A-P-M-C",
            CoTType::GndPublicaffairsjibtheater => "a-.-G-U-S-A-P-M-T",
            CoTType::GndPublicaffairstheater => "a-.-G-U-S-A-P-T",
            CoTType::GndQuartermasterSupply => "a-.-G-U-S-A-Q",
            CoTType::GndQuartermasterSupplyCorps => "a-.-G-U-S-A-Q-C",
            CoTType::GndQuartermasterSupplyTheater => "a-.-G-U-S-A-Q-T",
            CoTType::GndReligiousChaplain => "a-.-G-U-S-A-R",
            CoTType::GndReligiousChaplaincorps => "a-.-G-U-S-A-R-C",
            CoTType::GndReligiousChaplaintheater => "a-.-G-U-S-A-R-T",
            CoTType::GndPersonnelservices => "a-.-G-U-S-A-S",
            CoTType::GndPersonnelcorps => "a-.-G-U-S-A-S-C",
            CoTType::GndPersonneltheater => "a-.-G-U-S-A-S-T",
            CoTType::GndAdmintheater => "a-.-G-U-S-A-T",
            CoTType::GndMoraleWelfareRecreationMwr => "a-.-G-U-S-A-W",
            CoTType::GndMwrcorps => "a-.-G-U-S-A-W-C",
            CoTType::GndMwrtheater => "a-.-G-U-S-A-W-T",
            CoTType::GndReplacementholdingunitRhu => "a-.-G-U-S-A-X",
            CoTType::GndRhucorps => "a-.-G-U-S-A-X-C",
            CoTType::GndRhutheater => "a-.-G-U-S-A-X-T",
            CoTType::GndMedical => "a-.-G-U-S-M",
            CoTType::GndMedicalcorps => "a-.-G-U-S-M-C",
            CoTType::GndMedicaldental => "a-.-G-U-S-M-D",
            CoTType::GndMedicaldentalcorps => "a-.-G-U-S-M-D-C",
            CoTType::GndMedicaldentaltheater => "a-.-G-U-S-M-D-T",
            CoTType::GndMedicaltreatmentfacility => "a-.-G-U-S-M-M",
            CoTType::GndMedicaltreatmentfacilitycorps => "a-.-G-U-S-M-M-C",
            CoTType::GndMedicaltreatmentfacilitytheater => "a-.-G-U-S-M-M-T",
            CoTType::GndMedicalpsychological => "a-.-G-U-S-M-P",
            CoTType::GndMedicalpsychologicalcorps => "a-.-G-U-S-M-P-C",
            CoTType::GndMedicalpsychologicaltheater => "a-.-G-U-S-M-P-T",
            CoTType::GndMedicaltheater => "a-.-G-U-S-M-T",
            CoTType::GndMedicalveterinary => "a-.-G-U-S-M-V",
            CoTType::GndMedicalveterinarycorps => "a-.-G-U-S-M-V-C",
            CoTType::GndMedicalveterinarytheater => "a-.-G-U-S-M-V-T",
            CoTType::GndSupply => "a-.-G-U-S-S",
            CoTType::GndSupplyclassi => "a-.-G-U-S-S-1",
            CoTType::GndSupplyclassicorps => "a-.-G-U-S-S-1-C",
            CoTType::GndSupplyclassitheater => "a-.-G-U-S-S-1-T",
            CoTType::GndSupplyclassii => "a-.-G-U-S-S-2",
            CoTType::GndSupplyclassiicorps => "a-.-G-U-S-S-2-C",
            CoTType::GndSupplyclassiitheater => "a-.-G-U-S-S-2-T",
            CoTType::GndSupplyclassiii => "a-.-G-U-S-S-3",
            CoTType::GndSupplyclassiiiaviation => "a-.-G-U-S-S-3-A",
            CoTType::GndSupplyclassiiiaviationcorps => "a-.-G-U-S-S-3-A-C",
            CoTType::GndSupplyclassiiiaviationtheater => "a-.-G-U-S-S-3-A-T",
            CoTType::GndSupplyclassiiicorps => "a-.-G-U-S-S-3-C",
            CoTType::GndSupplyclassiiitheater => "a-.-G-U-S-S-3-T",
            CoTType::GndSupplyclassiv => "a-.-G-U-S-S-4",
            CoTType::GndSupplyclassivcorps => "a-.-G-U-S-S-4-C",
            CoTType::GndSupplyclassivtheater => "a-.-G-U-S-S-4-T",
            CoTType::GndSupplyclassv => "a-.-G-U-S-S-5",
            CoTType::GndSupplyclassvcorps => "a-.-G-U-S-S-5-C",
            CoTType::GndSupplyclassvtheater => "a-.-G-U-S-S-5-T",
            CoTType::GndSupplyclassvi => "a-.-G-U-S-S-6",
            CoTType::GndSupplyclassvicorps => "a-.-G-U-S-S-6-C",
            CoTType::GndSupplyclassvitheater => "a-.-G-U-S-S-6-T",
            CoTType::GndSupplyclassvii => "a-.-G-U-S-S-7",
            CoTType::GndSupplyclassviicorps => "a-.-G-U-S-S-7-C",
            CoTType::GndSupplyclassviitheater => "a-.-G-U-S-S-7-T",
            CoTType::GndSupplyclassviii => "a-.-G-U-S-S-8",
            CoTType::GndSupplyclassviiicorps => "a-.-G-U-S-S-8-C",
            CoTType::GndSupplyclassviiitheater => "a-.-G-U-S-S-8-T",
            CoTType::GndSupplyclassix => "a-.-G-U-S-S-9",
            CoTType::GndSupplyclassixcorps => "a-.-G-U-S-S-9-C",
            CoTType::GndSupplyclassixtheater => "a-.-G-U-S-S-9-T",
            CoTType::GndSupplycorps => "a-.-G-U-S-S-C",
            CoTType::GndSupplylaundryBath => "a-.-G-U-S-S-L",
            CoTType::GndSupplylaundryBathcorps => "a-.-G-U-S-S-L-C",
            CoTType::GndSupplylaundryBaththeater => "a-.-G-U-S-S-L-T",
            CoTType::GndSupplytheater => "a-.-G-U-S-S-T",
            CoTType::GndSupplywater => "a-.-G-U-S-S-W",
            CoTType::GndSupplywatercorps => "a-.-G-U-S-S-W-C",
            CoTType::GndSupplywaterpurification => "a-.-G-U-S-S-W-P",
            CoTType::GndSupplywaterpurificationcorps => "a-.-G-U-S-S-W-P-C",
            CoTType::GndSupplywaterpurificationtheater => "a-.-G-U-S-S-W-P-T",
            CoTType::GndSupplywatertheater => "a-.-G-U-S-S-W-T",
            CoTType::GndSupplyclassx => "a-.-G-U-S-S-X",
            CoTType::GndSupplyclassxcorps => "a-.-G-U-S-S-X-C",
            CoTType::GndSupplyclassxtheater => "a-.-G-U-S-S-X-T",
            CoTType::GndSupplydepot => "a-.-G-U-S-S-d",
            CoTType::GndTransportation => "a-.-G-U-S-T",
            CoTType::GndApodApoe => "a-.-G-U-S-T-A",
            CoTType::GndApodApoecorps => "a-.-G-U-S-T-A-C",
            CoTType::GndApodApoetheater => "a-.-G-U-S-T-A-T",
            CoTType::GndTransportationcorps => "a-.-G-U-S-T-C",
            CoTType::GndMissile => "a-.-G-U-S-T-I",
            CoTType::GndMissilecorps => "a-.-G-U-S-T-I-C",
            CoTType::GndMissiletheater => "a-.-G-U-S-T-I-T",
            CoTType::GndMovementcontrolcenterMcc => "a-.-G-U-S-T-M",
            CoTType::GndMcccorps => "a-.-G-U-S-T-M-C",
            CoTType::GndMcctheater => "a-.-G-U-S-T-M-T",
            CoTType::GndRailhead => "a-.-G-U-S-T-R",
            CoTType::GndRailheadcorps => "a-.-G-U-S-T-R-C",
            CoTType::GndRailheadtheater => "a-.-G-U-S-T-R-T",
            CoTType::GndSpodSpoe => "a-.-G-U-S-T-S",
            CoTType::GndSpodSpoecorps => "a-.-G-U-S-T-S-C",
            CoTType::GndSpodSpoetheater => "a-.-G-U-S-T-S-T",
            CoTType::GndTransportationtheater => "a-.-G-U-S-T-T",
            CoTType::GndMaintenance => "a-.-G-U-S-X",
            CoTType::GndMaintenancecorps => "a-.-G-U-S-X-C",
            CoTType::GndElectrooptical => "a-.-G-U-S-X-E",
            CoTType::GndElectroopticalcorps => "a-.-G-U-S-X-E-C",
            CoTType::GndElectroopticaltheater => "a-.-G-U-S-X-E-T",
            CoTType::GndMaintenanceheavy => "a-.-G-U-S-X-H",
            CoTType::GndMaintenanceheavycorps => "a-.-G-U-S-X-H-C",
            CoTType::GndMaintenanceheavytheater => "a-.-G-U-S-X-H-T",
            CoTType::GndOrdnance => "a-.-G-U-S-X-O",
            CoTType::GndOrdnancecorps => "a-.-G-U-S-X-O-C",
            CoTType::GndOrdnancemissile => "a-.-G-U-S-X-O-M",
            CoTType::GndOrdnancemissilecorps => "a-.-G-U-S-X-O-M-C",
            CoTType::GndOrdnancemissiletheater => "a-.-G-U-S-X-O-M-T",
            CoTType::GndOrdnancetheater => "a-.-G-U-S-X-O-T",
            CoTType::GndMaintenancerecovery => "a-.-G-U-S-X-R",
            CoTType::GndMaintenancerecoverycorps => "a-.-G-U-S-X-R-C",
            CoTType::GndMaintenancerecoverytheater => "a-.-G-U-S-X-R-T",
            CoTType::GndMaintenancetheater => "a-.-G-U-S-X-T",
            CoTType::GndCombatsupport => "a-.-G-U-U",
            CoTType::GndCombatsupportnbc => "a-.-G-U-U-A",
            CoTType::GndBiological => "a-.-G-U-U-A-B",
            CoTType::GndReconequipped => "a-.-G-U-U-A-B-R",
            CoTType::GndChemical => "a-.-G-U-U-A-C",
            CoTType::GndSmokeDecon => "a-.-G-U-U-A-C-C",
            CoTType::GndMechanizedsmokeDecon => "a-.-G-U-U-A-C-C-K",
            CoTType::GndMotorizedsmokeDecon => "a-.-G-U-U-A-C-C-M",
            CoTType::GndChemicalrecon => "a-.-G-U-U-A-C-R",
            CoTType::GndChemicalwheeledarmoredvehicle => "a-.-G-U-U-A-C-R-S",
            CoTType::GndSmoke => "a-.-G-U-U-A-C-S",
            CoTType::GndArmorsmoke => "a-.-G-U-U-A-C-S-A",
            CoTType::GndMotorizedsmoke => "a-.-G-U-U-A-C-S-M",
            CoTType::GndDecontamination => "a-.-G-U-U-A-D",
            CoTType::GndNuclear => "a-.-G-U-U-A-N",
            CoTType::GndExplosiveordinancedisposal => "a-.-G-U-U-E",
            CoTType::GndInformationwarfareunit => "a-.-G-U-U-I",
            CoTType::GndLawenforcementunit => "a-.-G-U-U-L",
            CoTType::GndCivilianlawenforcement => "a-.-G-U-U-L-C",
            CoTType::GndCentralintelligencedivisionCid => "a-.-G-U-U-L-D",
            CoTType::GndSecuritypoliceAir => "a-.-G-U-U-L-F",
            CoTType::GndMilitarypolice => "a-.-G-U-U-L-M",
            CoTType::GndShorepatrol => "a-.-G-U-U-L-S",
            CoTType::GndMilitaryintelligence => "a-.-G-U-U-M",
            CoTType::GndAerialexploitation => "a-.-G-U-U-M-A",
            CoTType::GndCounterintelligence => "a-.-G-U-U-M-C",
            CoTType::GndJointintelligencecenter => "a-.-G-U-U-M-J",
            CoTType::GndMeteorological => "a-.-G-U-U-M-M-O",
            CoTType::GndOperations => "a-.-G-U-U-M-O",
            CoTType::GndInterrogation => "a-.-G-U-U-M-Q",
            CoTType::GndSurveillance => "a-.-G-U-U-M-R",
            CoTType::GndGroundsurveillanceradar => "a-.-G-U-U-M-R-G",
            CoTType::GndSensor => "a-.-G-U-U-M-R-S",
            CoTType::GndSensorscm => "a-.-G-U-U-M-R-S-S",
            CoTType::GndGroundstationmodule => "a-.-G-U-U-M-R-X",
            CoTType::GndSignalintelligenceSigint => "a-.-G-U-U-M-S",
            CoTType::GndElectronicwarfare => "a-.-G-U-U-M-S-E",
            CoTType::GndArmoredwheeledvehicle => "a-.-G-U-U-M-S-E-A",
            CoTType::GndCorps => "a-.-G-U-U-M-S-E-C",
            CoTType::GndDirectionfinding => "a-.-G-U-U-M-S-E-D",
            CoTType::GndIntercept => "a-.-G-U-U-M-S-E-I",
            CoTType::GndJamming => "a-.-G-U-U-M-S-E-J",
            CoTType::GndTheater => "a-.-G-U-U-M-S-E-T",
            CoTType::GndTacticalexploit => "a-.-G-U-U-M-T",
            CoTType::GndLandingsupport => "a-.-G-U-U-P",
            CoTType::GndSignalunit => "a-.-G-U-U-S",
            CoTType::GndArea => "a-.-G-U-U-S-A",
            CoTType::GndCommunicationconfiguredpackage => "a-.-G-U-U-S-C",
            CoTType::GndLargecommunicationconfiguredpackage => "a-.-G-U-U-S-C-L",
            CoTType::GndForwardcommunications => "a-.-G-U-U-S-F",
            CoTType::GndMultiplesubscriberelement => "a-.-G-U-U-S-M",
            CoTType::GndLargeextensionnode => "a-.-G-U-U-S-M-L",
            CoTType::GndNodecenter => "a-.-G-U-U-S-M-N",
            CoTType::GndSmallextensionnode => "a-.-G-U-U-S-M-S",
            CoTType::GndCommandoperations => "a-.-G-U-U-S-O",
            CoTType::GndRadiounit => "a-.-G-U-U-S-R",
            CoTType::GndTacticalsatellite => "a-.-G-U-U-S-R-S",
            CoTType::GndTeletypecenter => "a-.-G-U-U-S-R-T",
            CoTType::GndRelay => "a-.-G-U-U-S-R-W",
            CoTType::GndSignalsupport => "a-.-G-U-U-S-S",
            CoTType::GndTelephoneswitch => "a-.-G-U-U-S-W",
            CoTType::GndElectronicranging => "a-.-G-U-U-S-X",
            CoTType::GndImresources => "a-.-G-U-i",
            CoTType::GndImresourcesAnimalhealth => "a-.-G-U-i-a",
            CoTType::GndLargeanimalrescuestriketeam => "a-.-G-U-i-a-lar",
            CoTType::GndLargeanimalshelteringteam => "a-.-G-U-i-a-las",
            CoTType::GndLargeanimaltransportteam => "a-.-G-U-i-a-lat",
            CoTType::GndSmallanimalrescuestriketeam => "a-.-G-U-i-a-sar",
            CoTType::GndSmallanimalshelteringteam => "a-.-G-U-i-a-sas",
            CoTType::GndSmallanimaltransportteam => "a-.-G-U-i-a-sat",
            CoTType::GndIncidentmanagementteamanimalprotection => "a-.-G-U-i-a-imt",
            CoTType::GndImresourcesEmergencymanagement => "a-.-G-U-i-e",
            CoTType::GndAirbornecommunicationsrelayCap => "a-.-G-U-i-e-acrc",
            CoTType::GndAirbornecommunicationsrelayteam => "a-.-G-U-i-e-acrt",
            CoTType::GndAirbornetransportteam => "a-.-G-U-i-e-att",
            CoTType::GndCriticalincidentstressmanagementteam => "a-.-G-U-i-e-cis",
            CoTType::GndCommunicationssupportteam => "a-.-G-U-i-e-cst",
            CoTType::GndDonationscoordinator => "a-.-G-U-i-e-dc",
            CoTType::GndDonationsmanagementpersonnelteam => "a-.-G-U-i-e-dmt",
            CoTType::GndEvacuationcoordinationteam => "a-.-G-U-i-e-ect",
            CoTType::GndEocfinanceadministration => "a-.-G-U-i-e-efa",
            CoTType::GndEvacuationliaisonteam => "a-.-G-U-i-e-elt",
            CoTType::GndEocmanagementsupportteam => "a-.-G-U-i-e-ems",
            CoTType::GndEocoperationssectionchief => "a-.-G-U-i-e-eos",
            CoTType::GndEocplanningsectionchief => "a-.-G-U-i-e-eps",
            CoTType::GndIndividualassistancedisasterassessmentteamleader => "a-.-G-U-i-e-ial",
            CoTType::GndIndividualassistancedisasterassessmentteam => "a-.-G-U-i-e-iat",
            CoTType::GndIncidentmanagementteam => "a-.-G-U-i-e-imt",
            CoTType::GndMobilecommunicationscenter => "a-.-G-U-i-e-mcc",
            CoTType::GndMobilefeedingkitchen => "a-.-G-U-i-e-mfk",
            CoTType::GndPublicassistancecoordinator => "a-.-G-U-i-e-pac",
            CoTType::GndRapidneedsassessmentteam => "a-.-G-U-i-e-rna",
            CoTType::GndSheltermanagementteam => "a-.-G-U-i-e-smt",
            CoTType::GndVolunteeragencyliaison => "a-.-G-U-i-e-val",
            CoTType::GndImresourcesEmergencymedicalservices => "a-.-G-U-i-m",
            CoTType::GndAirambulanceFixedWing => "a-.-G-U-i-m-aaf",
            CoTType::GndAirambulanceRotaryWing => "a-.-G-U-i-m-aar",
            CoTType::GndAmbulancesGround => "a-.-G-U-i-m-ag",
            CoTType::GndAmbulancestriketeam => "a-.-G-U-i-m-ast",
            CoTType::GndAmbulancetaskforce => "a-.-G-U-i-m-atf",
            CoTType::GndEmergencymedicaltaskforce => "a-.-G-U-i-m-etf",
            CoTType::GndImresourcesFirehazmat => "a-.-G-U-i-f",
            CoTType::GndAreacommandteamFirefighting => "a-.-G-U-i-f-act",
            CoTType::GndBrushpatrol => "a-.-G-U-i-f-bp",
            CoTType::GndCrewtransport => "a-.-G-U-i-f-ct",
            CoTType::GndEngineFirePumper => "a-.-G-U-i-f-efp",
            CoTType::GndFireboat => "a-.-G-U-i-f-fb",
            CoTType::GndFueltender => "a-.-G-U-i-f-ft",
            CoTType::GndFiretruck => "a-.-G-U-i-f-fta",
            CoTType::GndFoamtender => "a-.-G-U-i-f-ftf",
            CoTType::GndHandcrew => "a-.-G-U-i-f-hc",
            CoTType::GndHazmatentryteam => "a-.-G-U-i-f-het",
            CoTType::GndHelicoptersfirefighting => "a-.-G-U-i-f-hf",
            CoTType::GndHelitanker => "a-.-G-U-i-f-ht",
            CoTType::GndIncidentmanagementteamfirefighting => "a-.-G-U-i-f-imt",
            CoTType::GndInteragencybuyingteamfirefighting => "a-.-G-U-i-f-ibt",
            CoTType::GndMobilecommunicationsunit => "a-.-G-U-i-f-mcu",
            CoTType::GndUscgnationalstrikeforce => "a-.-G-U-i-f-nsf",
            CoTType::GndPortablepump => "a-.-G-U-i-f-pp",
            CoTType::GndStriketeamEngineFire => "a-.-G-U-i-f-st",
            CoTType::GndWatertenderFirefightingTanker => "a-.-G-U-i-f-wt",
            CoTType::GndImresourcesHealthandmedical => "a-.-G-U-i-h",
            CoTType::GndDisastermedicalassistanceteamBasic => "a-.-G-U-i-h-dmb",
            CoTType::GndDisastermedicalassistanceteamCrushinjuryspecialty => "a-.-G-U-i-h-dmc",
            CoTType::GndDisastermedicalassistanceteamMentalhealth => "a-.-G-U-i-h-dmm",
            CoTType::GndDisastermedicalassistanceteamPediatric => "a-.-G-U-i-h-dmp",
            CoTType::GndDisastermedicalassistanceteamBurn => "a-.-G-U-i-h-dms",
            CoTType::GndDisastermortuaryoperationalresponseteam => "a-.-G-U-i-h-mor",
            CoTType::GndInternationalmedicalsurgicalresponseteam => "a-.-G-U-i-h-msr",
            CoTType::GndNdmsmanagementsupportteam => "a-.-G-U-i-h-mst",
            CoTType::GndVeterinarymedicalassistanceteam => "a-.-G-U-i-h-vma",
            CoTType::GndImresourcesLawenforcement => "a-.-G-U-i-l",
            CoTType::GndBombsquadexplosivesteam => "a-.-G-U-i-l-bs",
            CoTType::GndCrowdcontrolteam => "a-.-G-U-i-l-cct",
            CoTType::GndPublicsafetydiveteam => "a-.-G-U-i-l-dt",
            CoTType::GndAviationHelicopterspatrolsurveillance => "a-.-G-U-i-l-hps",
            CoTType::GndObservationaircraft => "a-.-G-U-i-l-oa",
            CoTType::GndSwattacticalteam => "a-.-G-U-i-l-tt",
            CoTType::GndImresourcesPublicworks => "a-.-G-U-i-p",
            CoTType::GndAircurtainburnersAboveground => "a-.-G-U-i-p-acf",
            CoTType::GndAirconditionerheater => "a-.-G-U-i-p-ach",
            CoTType::GndAircurtainburnersInground => "a-.-G-U-i-p-act",
            CoTType::GndAllterraincranes => "a-.-G-U-i-p-atc",
            CoTType::GndBackhoeloader => "a-.-G-U-i-p-bh",
            CoTType::GndChillersairhandlers => "a-.-G-U-i-p-cah",
            CoTType::GndCrawlercranes => "a-.-G-U-i-p-cc",
            CoTType::GndConcretecuttermultiProcessorforhydraulicexcavator => "a-.-G-U-i-p-cce",
            CoTType::GndDisasterassessmentteam => "a-.-G-U-i-p-dat",
            CoTType::GndDebrismanagementmonitoringteam => "a-.-G-U-i-p-dmm",
            CoTType::GndDebrismanagementsitereductionteam => "a-.-G-U-i-p-dmr",
            CoTType::GndDebrismanagementteam => "a-.-G-U-i-p-dmt",
            CoTType::GndDisasterrecoveryteam => "a-.-G-U-i-p-drt",
            CoTType::GndDumptrailer => "a-.-G-U-i-p-dt",
            CoTType::GndDumptruckoffroad => "a-.-G-U-i-p-dtf",
            CoTType::GndDumptruckonroad => "a-.-G-U-i-p-dtn",
            CoTType::GndElectricalpowerrestorationteam => "a-.-G-U-i-p-epr",
            CoTType::GndEngineeringservices => "a-.-G-U-i-p-es",
            CoTType::GndFlatbedtrailertruck => "a-.-G-U-i-p-fbt",
            CoTType::GndGenerators => "a-.-G-U-i-p-gen",
            CoTType::GndHydraulicexcavatorLarge => "a-.-G-U-i-p-hel",
            CoTType::GndHydraulicexcavatorMedium => "a-.-G-U-i-p-hem",
            CoTType::GndHydraulictruckcranes => "a-.-G-U-i-p-htc",
            CoTType::GndLatticetruckcranes => "a-.-G-U-i-p-ltc",
            CoTType::GndTugboat => "a-.-G-U-i-p-tb",
            CoTType::GndTrackdozer => "a-.-G-U-i-p-td",
            CoTType::GndTubgrinder => "a-.-G-U-i-p-tg",
            CoTType::GndTractortrailer => "a-.-G-U-i-p-tt",
            CoTType::GndWheeldozer => "a-.-G-U-i-p-wd",
            CoTType::GndWheelloadersLarge => "a-.-G-U-i-p-wll",
            CoTType::GndWheelloadersMedium => "a-.-G-U-i-p-wlm",
            CoTType::GndWheelloadersSmall => "a-.-G-U-i-p-wls",
            CoTType::GndWaterpurificationteam => "a-.-G-U-i-p-wpt",
            CoTType::GndWatertruck => "a-.-G-U-i-p-wt",
            CoTType::GndImresourcesSearchandrescue => "a-.-G-U-i-s",
            CoTType::GndAirbornereconnaissance => "a-.-G-U-i-s-ar",
            CoTType::GndAirsearchteam => "a-.-G-U-i-s-ast",
            CoTType::GndCaninesarteamavalanchesnow => "a-.-G-U-i-s-cas",
            CoTType::GndCavesarteam => "a-.-G-U-i-s-cav",
            CoTType::GndCaninesarteamdisasterresponse => "a-.-G-U-i-s-cdr",
            CoTType::GndCaninesarteamlandcadaver => "a-.-G-U-i-s-clc",
            CoTType::GndCollapsesarteam => "a-.-G-U-i-s-col",
            CoTType::GndCaninesarteamwater => "a-.-G-U-i-s-cwa",
            CoTType::GndCaninesarteamwilderness => "a-.-G-U-i-s-cwi",
            CoTType::GndCaninesarteamwildernesstracking => "a-.-G-U-i-s-cwt",
            CoTType::GndMinetunnelsarteam => "a-.-G-U-i-s-mt",
            CoTType::GndMountainsarteam => "a-.-G-U-i-s-mnt",
            CoTType::GndRadiodirectionfindingteam => "a-.-G-U-i-s-rdf",
            CoTType::GndSwiftwaterfloodsearchanddiverescueteam => "a-.-G-U-i-s-sfd",
            CoTType::GndUsarincidentsupportteam => "a-.-G-U-i-s-uis",
            CoTType::GndUsartaskforce => "a-.-G-U-i-s-utf",
            CoTType::GndWildernesssarteam => "a-.-G-U-i-s-wi",
            CoTType::GndImresourcesOther => "a-.-G-U-i-o",
            CoTType::SurfaceSeasurfacetrack => "a-.-S",
            CoTType::SurfaceCombatant => "a-.-S-C",
            CoTType::SurfaceAmphibiouswarfareship => "a-.-S-C-A",
            CoTType::SurfaceAssaultvessel => "a-.-S-C-A-L-A",
            CoTType::SurfaceLandingcraft => "a-.-S-C-A-L-C",
            CoTType::SurfaceLandingship => "a-.-S-C-A-L-S",
            CoTType::SurfaceHovercraft => "a-.-S-C-H",
            CoTType::SurfaceLine => "a-.-S-C-L",
            CoTType::SurfaceBattleship => "a-.-S-C-L-B-B",
            CoTType::SurfaceCruiser => "a-.-S-C-L-C-C",
            CoTType::SurfaceCarrier => "a-.-S-C-L-C-V",
            CoTType::SurfaceDestroyer => "a-.-S-C-L-D-D",
            CoTType::SurfaceFrigateCorvette => "a-.-S-C-L-F-F",
            CoTType::SurfaceMinewarfarevessel => "a-.-S-C-M",
            CoTType::SurfaceMcmsupport => "a-.-S-C-M-M-A",
            CoTType::SurfaceMcmdrone => "a-.-S-C-M-M-D",
            CoTType::SurfaceMinehunter => "a-.-S-C-M-M-H",
            CoTType::SurfaceMinelayer => "a-.-S-C-M-M-L",
            CoTType::SurfaceMinesweeper => "a-.-S-C-M-M-S",
            CoTType::SurfacePatrol => "a-.-S-C-P",
            CoTType::SurfaceAntisubmarinewarfare => "a-.-S-C-P-S-B",
            CoTType::SurfaceAntisurfacewarfare => "a-.-S-C-P-S-U",
            CoTType::SurfaceNavygroup => "a-.-S-G",
            CoTType::SurfaceConvoy => "a-.-S-G-C",
            CoTType::SurfaceNavytaskgroup => "a-.-S-G-G",
            CoTType::SurfaceNavytaskforce => "a-.-S-G-T",
            CoTType::SurfaceNavytaskunit => "a-.-S-G-U",
            CoTType::SurfaceNoncombatant => "a-.-S-N",
            CoTType::SurfaceFleetsupport => "a-.-S-N-F",
            CoTType::SurfaceIntelligence => "a-.-S-N-I",
            CoTType::SurfaceHospitalship => "a-.-S-N-M",
            CoTType::SurfaceStation => "a-.-S-N-N",
            CoTType::SurfaceRescue => "a-.-S-N-N-R",
            CoTType::SurfaceUnderwayreplenishment => "a-.-S-N-R",
            CoTType::SurfaceServiceSupportharbor => "a-.-S-N-S",
            CoTType::SurfaceOwntrack => "a-.-S-O",
            // CoTType::SurfaceStation => "a-.-S-S",
            CoTType::SurfaceAswship => "a-.-S-S-A",
            CoTType::SurfacePicket => "a-.-S-S-P",
            CoTType::SurfaceNonmilitary => "a-.-S-X",
            CoTType::SurfaceFishing => "a-.-S-X-F",
            CoTType::SurfaceDrifter => "a-.-S-X-F-D-F",
            CoTType::SurfaceDredge => "a-.-S-X-F-D-R",
            CoTType::SurfaceTrawler => "a-.-S-X-F-T-R",
            CoTType::SurfaceLawenforcementvessel => "a-.-S-X-L",
            CoTType::SurfaceMerchant => "a-.-S-X-M",
            CoTType::SurfaceCargo => "a-.-S-X-M-C",
            CoTType::SurfaceFerry => "a-.-S-X-M-F",
            CoTType::SurfaceHazardousmaterialsHazmat => "a-.-S-X-M-H",
            CoTType::SurfaceOilerTanker => "a-.-S-X-M-O",
            CoTType::SurfacePassenger => "a-.-S-X-M-P",
            CoTType::SurfaceRollonRolloff => "a-.-S-X-M-R",
            CoTType::SurfaceTowingvessel => "a-.-S-X-M-T-O",
            CoTType::SurfaceTug => "a-.-S-X-M-T-U",
            CoTType::SurfaceLeisurecraft => "a-.-S-X-R",
            CoTType::SubsurfSubsurfacetrack => "a-.-U",
            CoTType::SubsurfNonsubmarine => "a-.-U-N",
            CoTType::SubsurfDiver => "a-.-U-N-D",
            CoTType::SubsurfSubmarine => "a-.-U-S",
            CoTType::SubsurfSubmarineconventionalpropulsion => "a-.-U-S-C",
            CoTType::SubsurfSubmarinenuclearpropulsion => "a-.-U-S-N",
            CoTType::SubsurfOthersubmersible => "a-.-U-S-O",
            CoTType::SubsurfStation => "a-.-U-S-S",
            CoTType::SubsurfAswsubmarine => "a-.-U-S-S-A",
            CoTType::SubsurfUnmannedunderwatervehicleUuv => "a-.-U-S-U",
            CoTType::SubsurfUnderwaterweapon => "a-.-U-W",
            CoTType::SubsurfUnderwaterdecoy => "a-.-U-W-D",
            CoTType::SubsurfSeaminedecoy => "a-.-U-W-D-M",
            CoTType::SubsurfSeamine => "a-.-U-W-M",
            CoTType::SubsurfSeaminedealt => "a-.-U-W-M-D",
            CoTType::SubsurfSeamineFloating => "a-.-U-W-M-F",
            CoTType::SubsurfSeamineFloatingDealt => "a-.-U-W-M-F-D",
            CoTType::SubsurfSeamineGround => "a-.-U-W-M-G",
            CoTType::SubsurfSeamineGroundDealt => "a-.-U-W-M-G-D",
            CoTType::SubsurfSeamineMoored => "a-.-U-W-M-M",
            CoTType::SubsurfSeamineMooredDealt => "a-.-U-W-M-M-D",
            CoTType::SubsurfSeamineInotherposition => "a-.-U-W-M-O",
            CoTType::SubsurfSeamineInotherpositionDealt => "a-.-U-W-M-O-D",
            CoTType::SubsurfTorpedo => "a-.-U-W-T",
            CoTType::Other => "a-.-X",
            CoTType::OtherIncident => "a-.-X-i",
            CoTType::OtherIncidentGeo => "a-.-X-i-g",
            CoTType::OtherIncidentGeoAvalanche => "a-.-X-i-g-a",
            CoTType::OtherIncidentGeoEarthquake => "a-.-X-i-g-e",
            CoTType::OtherIncidentGeoAftershock => "a-.-X-i-g-e-a",
            CoTType::OtherIncidentGeoEpicenter => "a-.-X-i-g-e-e",
            CoTType::OtherIncidentGeoLandslide => "a-.-X-i-g-l",
            CoTType::OtherIncidentGeoSubsistance => "a-.-X-i-g-s",
            CoTType::OtherIncidentGeoVolcano => "a-.-X-i-g-v",
            CoTType::OtherIncidentGeoEruption => "a-.-X-i-g-v-e",
            CoTType::OtherIncidentGeoVolcanothreat => "a-.-X-i-g-v-t",
            CoTType::OtherIncidentMet => "a-.-X-i-m",
            CoTType::OtherIncidentMetDrizzle => "a-.-X-i-m-z",
            CoTType::OtherIncidentMetDrought => "a-.-X-i-m-d",
            CoTType::OtherIncidentMetFlood => "a-.-X-i-m-f",
            CoTType::OtherIncidentMetFog => "a-.-X-i-m-g",
            CoTType::OtherIncidentMetHail => "a-.-X-i-m-h",
            CoTType::OtherIncidentMetInversion => "a-.-X-i-m-i",
            CoTType::OtherIncidentMetRain => "a-.-X-i-m-r",
            CoTType::OtherIncidentMetSandstorm => "a-.-X-i-m-a",
            CoTType::OtherIncidentMetSnow => "a-.-X-i-m-s",
            CoTType::OtherIncidentMetthunderstorm => "a-.-X-i-m-u",
            CoTType::OtherIncidentMetTornado => "a-.-X-i-m-t",
            CoTType::OtherIncidentMetCyclone => "a-.-X-i-m-c",
            CoTType::OtherIncidentMetTsunami => "a-.-X-i-m-n",
            CoTType::OtherIncidentSafety => "a-.-X-i-s",
            CoTType::OtherIncidentSecurity => "a-.-X-i-l",
            CoTType::OtherIncidentSecurityCivildisturb => "a-.-X-i-l-c",
            CoTType::OtherIncidentSecurityCivildisturbDemonstration => "a-.-X-i-l-c-d",
            CoTType::OtherIncidentSecurityCivildisturbDisplaced => "a-.-X-i-l-c-p",
            CoTType::OtherIncidentSecurityCivildisturbRioting => "a-.-X-i-l-c-r",
            CoTType::OtherIncidentSecurityLawenforcement => "a-.-X-i-l-l",
            CoTType::OtherIncidentSecurityLawenforcementBombing => "a-.-X-i-l-l-b",
            CoTType::OtherIncidentSecurityLawenforcementBombexplosion => "a-.-X-i-l-l-b-e",
            CoTType::OtherIncidentSecurityLawenforcementbombthreat => "a-.-X-i-l-l-b-t",
            CoTType::OtherIncidentSecurityLawenforcementLooting => "a-.-X-i-l-l-l",
            CoTType::OtherIncidentSecurityLawenforcementPoisoning => "a-.-X-i-l-l-p",
            CoTType::OtherIncidentSecurityLawenforcementShooting => "a-.-X-i-l-l-s",
            CoTType::OtherIncidentRescue => "a-.-X-i-r",
            CoTType::OtherIncidentFire => "a-.-X-i-f",
            CoTType::OtherIncidentFireWild => "a-.-X-i-f-w",
            CoTType::OtherIncidentFireNonresidential => "a-.-X-i-f-n",
            CoTType::OtherIncidentFireNonresidentialspecialneed => "a-.-X-i-f-n-h",
            CoTType::OtherIncidentFireNonresidentialschool => "a-.-X-i-f-n-s",
            CoTType::OtherIncidentFireResidential => "a-.-X-i-f-r",
            CoTType::OtherIncidentFireHotspot => "a-.-X-i-f-h",
            CoTType::OtherIncidentFireOrigin => "a-.-X-i-f-o",
            CoTType::OtherIncidentFireSmoke => "a-.-X-i-f-s",
            CoTType::OtherIncidentFireSpecialneeds => "a-.-X-i-f-p",
            CoTType::OtherIncidentMedicalpublichealth => "a-.-X-i-h",
            CoTType::OtherIncidentPollutionenvironmental => "a-.-X-i-e",
            CoTType::OtherIncidentTransportation => "a-.-X-i-t",
            CoTType::OtherIncidentTransportationAir => "a-.-X-i-t-a",
            CoTType::OtherIncidentTransportationAiraccident => "a-.-X-i-t-a-a",
            CoTType::OtherIncidentTransportationAirhijacking => "a-.-X-i-t-a-h",
            CoTType::OtherIncidentTransportationMaritime => "a-.-X-i-t-m",
            CoTType::OtherIncidentTransportationMaritimeaccident => "a-.-X-i-t-m-a",
            CoTType::OtherIncidentTransportationMaritimehijacking => "a-.-X-i-t-m-h",
            CoTType::OtherIncidentTransportationRail => "a-.-X-i-t-r",
            CoTType::OtherIncidentTransportationRailaccident => "a-.-X-i-t-r-a",
            CoTType::OtherIncidentTransportationRailhijacking => "a-.-X-i-t-r-h",
            CoTType::OtherIncidentTransportationVehicle => "a-.-X-i-t-v",
            CoTType::OtherIncidentTransportationVehicleaccident => "a-.-X-i-t-v-a",
            CoTType::OtherIncidentTransportationVehiclehijacking => "a-.-X-i-t-v-h",
            CoTType::OtherIncidentInfrastructure => "a-.-X-i-i",
            CoTType::OtherIncidentCbrne => "a-.-X-i-c",
            CoTType::OtherIncidentOther => "a-.-X-i-o",
            CoTType::Maydaymessage => "a-.-.*-9-1-1",
        }
    }
}

/// Curson on Target classification
pub enum CotClassification {
    // pending
    Pending,
    Unknown,
    AssumedFriend,
    Friend,
    Neutral,
    Suspect,
    Hostile,
    Joker,
    Faker,
    NoneSpecified,
    Other,
}

// Clone CotClassification

impl Clone for CotClassification {
    fn clone(&self) -> CotClassification {
        match self {
            CotClassification::Pending => CotClassification::Pending,
            CotClassification::Unknown => CotClassification::Unknown,
            CotClassification::AssumedFriend => CotClassification::AssumedFriend,
            CotClassification::Friend => CotClassification::Friend,
            CotClassification::Neutral => CotClassification::Neutral,
            CotClassification::Suspect => CotClassification::Suspect,
            CotClassification::Hostile => CotClassification::Hostile,
            CotClassification::Joker => CotClassification::Joker,
            CotClassification::Faker => CotClassification::Faker,
            CotClassification::NoneSpecified => CotClassification::NoneSpecified,
            CotClassification::Other => CotClassification::Other,
        }
    }
}

impl Copy for CotClassification {}

/// Emun CoTType and string as tuple
/// * CoTType is the type of Cursor on Target
/// * String is the string representation of the CoTType
/// This is used to define the Cursor on Target types and their string representation
///
pub static COT_DEFINITIONS: &[(CoTType, &str)] = &[
    (CoTType::AirAirtrack, "a-.-A"),
    (CoTType::AirCiv, "a-.-A-C"),
    (CoTType::AirCivFixed, "a-.-A-C-F"),
    (CoTType::AirCivFixedRpvDroneUav, "a-.-A-C-F-q"),
    (CoTType::AirCivRotary, "a-.-A-C-H"),
    (CoTType::AirCivBlimp, "a-.-A-C-L"),
    (CoTType::AirMilFixed, "a-.-A-M-F"),
    (CoTType::AirMilFixedGunship, "a-.-A-M-F-g"),
    (CoTType::AirMilFixedAttackStrike, "a-.-A-M-F-A"),
    (CoTType::AirMilFixedBomber, "a-.-A-M-F-B"),
    (CoTType::AirMilFixedTransport, "a-.-A-M-F-C"),
    (CoTType::AirMilFixedTransportHeavy, "a-.-A-M-F-C-H"),
    (CoTType::AirMilFixedTransportLight, "a-.-A-M-F-C-L"),
    (CoTType::AirMilFixedTransportMedium, "a-.-A-M-F-C-M"),
    (CoTType::AirMilFixedC2, "a-.-A-M-F-D"),
    (CoTType::AirMilFixedFighter, "a-.-A-M-F-F"),
    (CoTType::AirMilFixedInterceptor, "a-.-A-M-F-F-I"),
    (CoTType::AirMilFixedCsar, "a-.-A-M-F-H"),
    (CoTType::AirMilFixedEcmJammer, "a-.-A-M-F-J"),
    (CoTType::AirMilFixedTanker, "a-.-A-M-F-K"),
    (CoTType::AirMilFixedVstol, "a-.-A-M-F-L"),
    (CoTType::AirMilFixedSof, "a-.-A-M-F-M"),
    (CoTType::AirMilFixedMedevac, "a-.-A-M-F-O"),
    (CoTType::AirMilFixedPatrol, "a-.-A-M-F-P"),
    (CoTType::AirMilFixedMinecountermeasures, "a-.-A-M-F-P-M"),
    (CoTType::AirMilFixedAsuw, "a-.-A-M-F-P-N"),
    (CoTType::AirMilFixedDroneRpvUav, "a-.-A-M-F-Q"),
    (CoTType::AirMilFixedReconn, "a-.-A-M-F-R"),
    (CoTType::AirMilFixedAew, "a-.-A-M-F-R-W"),
    (CoTType::AirMilFixedPhoto, "a-.-A-M-F-R-X"),
    (CoTType::AirMilFixedEsm, "a-.-A-M-F-R-Z"),
    (CoTType::AirMilFixedAswCarrierbased, "a-.-A-M-F-S"),
    (CoTType::AirMilFixedTrainer, "a-.-A-M-F-T"),
    (CoTType::AirMilFixedUtility, "a-.-A-M-F-U"),
    (CoTType::AirMilFixedUtilityHeavy, "a-.-A-M-F-U-H"),
    (CoTType::AirMilFixedUtilityLight, "a-.-A-M-F-U-L"),
    (CoTType::AirMilFixedUtilityMedium, "a-.-A-M-F-U-M"),
    (CoTType::AirMilFixedC3I, "a-.-A-M-F-Y"),
    (CoTType::AirMilRotor, "a-.-A-M-H"),
    (CoTType::AirMilRotorAttack, "a-.-A-M-H-A"),
    (CoTType::AirMilRotorTransport, "a-.-A-M-H-C"),
    (CoTType::AirMilRotorTransportHeavy, "a-.-A-M-H-C-H"),
    (CoTType::AirMilRotorTransportLight, "a-.-A-M-H-C-L"),
    (CoTType::AirMilRotorTransportMedium, "a-.-A-M-H-C-M"),
    (CoTType::AirMilRotorC2, "a-.-A-M-H-D"),
    (CoTType::AirMilRotorCsar, "a-.-A-M-H-H"),
    (CoTType::AirMilRotorMinecountermeasures, "a-.-A-M-H-I"),
    (CoTType::AirMilRotorEcmJammer, "a-.-A-M-H-J"),
    (CoTType::AirMilRotorTanker, "a-.-A-M-H-K"),
    (CoTType::AirMilRotorSof, "a-.-A-M-H-M"),
    (CoTType::AirMilRotorMedevac, "a-.-A-M-H-O"),
    (CoTType::AirMilRotorDroneRpvUav, "a-.-A-M-H-Q"),
    (CoTType::AirMilRotorReconnaissance, "a-.-A-M-H-R"),
    (CoTType::AirMilRotorAswMpa, "a-.-A-M-H-S"),
    (CoTType::AirMilRotorTrainer, "a-.-A-M-H-T"),
    (CoTType::AirMilRotorUtility, "a-.-A-M-H-U"),
    (CoTType::AirMilRotorUtilityHeavy, "a-.-A-M-H-U-H"),
    (CoTType::AirMilRotorUtilityLight, "a-.-A-M-H-U-L"),
    (CoTType::AirMilRotorUtilityMedium, "a-.-A-M-H-U-M"),
    (CoTType::AirMilBlimp, "a-.-A-M-L"),
    (CoTType::AirWeapon, "a-.-A-W"),
    (CoTType::AirWeaponDecoy, "a-.-A-W-D"),
    (CoTType::AirWeaponMissileinflight, "a-.-A-W-M"),
    (CoTType::AirWeaponMissileAirlaunched, "a-.-A-W-M-A"),
    (CoTType::AirWeaponMissileAirtoair, "a-.-A-W-M-A-A"),
    (CoTType::AirWeaponMissileAirtosurface, "a-.-A-W-M-A-S"),
    (CoTType::AirWeaponMissileAttackmissile, "a-.-A-W-M-L"),
    (CoTType::AirWeaponMissileLandlaunchedmissile, "a-.-A-W-M-S"),
    (CoTType::AirWeaponMissileSam, "a-.-A-W-M-S-A"),
    (CoTType::AirWeaponMissileSamFixedsite, "a-.-A-W-M-S-A-f"),
    (CoTType::AirWeaponMissileSamManpad, "a-.-A-W-M-S-A-i"),
    (CoTType::AirWeaponMissileSamMobile, "a-.-A-W-M-S-A-m"),
    (
        CoTType::AirWeaponMissileSurfacetosurfaceSsm,
        "a-.-A-W-M-S-S",
    ),
    (
        CoTType::AirWeaponMissileSubsurfacetosurfaceSSsm,
        "a-.-A-W-M-U",
    ),
    (CoTType::Ground, "a-.-G"),
    (CoTType::GndEquipment, "a-.-G-E"),
    (CoTType::GndEquipSensor, "a-.-G-E-S"),
    (CoTType::GndEquipSensorEmplaced, "a-.-G-E-S-E"),
    (CoTType::GndEquipSensorRadar, "a-.-G-E-S-R"),
    (CoTType::GndEquipVehicle, "a-.-G-E-V"),
    (CoTType::GndEquipVehicArmorGun, "a-.-G-E-V-A"),
    (CoTType::GndEquipVehicArmorApc, "a-.-G-E-V-A-A"),
    (CoTType::GndEquipVehicArmorApcRecovery, "a-.-G-E-V-A-A-R"),
    (CoTType::GndEquipVehicArmorC2VAcv, "a-.-G-E-V-A-C"),
    (CoTType::GndEquipVehicArmorInfantry, "a-.-G-E-V-A-I"),
    (CoTType::GndEquipVehicArmorLight, "a-.-G-E-V-A-L"),
    (
        CoTType::GndEquipVehicArmorCombatservicesupport,
        "a-.-G-E-V-A-S",
    ),
    (CoTType::GndEquipVehicArmorTank, "a-.-G-E-V-A-T"),
    (CoTType::GndEquipVehicArmorTankHeavy, "a-.-G-E-V-A-T-H"),
    (
        CoTType::GndEquipVehicArmorTankHeavyRecovery,
        "a-.-G-E-V-A-T-H-R",
    ),
    (CoTType::GndEquipVehicArmorTankLight, "a-.-G-E-V-A-T-L"),
    (
        CoTType::GndEquipVehicArmorTankLightRecovery,
        "a-.-G-E-V-A-T-L-R",
    ),
    (CoTType::GndEquipVehicArmorTankMedium, "a-.-G-E-V-A-T-M"),
    (
        CoTType::GndEquipVehicArmorTankMediumRecovery,
        "a-.-G-E-V-A-T-M-R",
    ),
    (CoTType::GndEquipVehicCivilian, "a-.-G-E-V-C"),
    (CoTType::GndEquipVehicEngineer, "a-.-G-E-V-E"),
    (CoTType::GndEquipVehicMineclearingvehicle, "a-.-G-E-V-E-A"),
    (
        CoTType::GndEquipVehicArmoredmountedmineclearingvehicle,
        "a-.-G-E-V-E-A-A",
    ),
    (
        CoTType::GndEquipVehicTrailermountedmineclearingvehicle,
        "a-.-G-E-V-E-A-T",
    ),
    (CoTType::GndEquipVehicBridge, "a-.-G-E-V-E-B"),
    (CoTType::GndEquipVehicConstruction, "a-.-G-E-V-E-C"),
    (CoTType::GndEquipVehicDozer, "a-.-G-E-V-E-D"),
    (CoTType::GndEquipVehicEarthmover, "a-.-G-E-V-E-E"),
    (CoTType::GndEquipVehicMinelayingvehicle, "a-.-G-E-V-E-M"),
    (
        CoTType::GndEquipVehicTruckmountedwithvolcano,
        "a-.-G-E-V-E-M-L",
    ),
    (
        CoTType::GndEquipVehicArmoredcarrierwithvolcano,
        "a-.-G-E-V-E-M-V",
    ),
    (CoTType::GndEquipVehicLocomotive, "a-.-G-E-V-T"),
    (CoTType::GndEquipVehicUtility, "a-.-G-E-V-U"),
    (CoTType::GndEquipVehicBus, "a-.-G-E-V-U-B"),
    (
        CoTType::GndEquipVehicLimitedcrosscountrytruck,
        "a-.-G-E-V-U-L",
    ),
    (CoTType::GndEquipVehicBoat, "a-.-G-E-V-U-R"),
    (CoTType::GndEquipVehicBoatLarge, "a-.-G-E-V-U-R-l"),
    (CoTType::GndEquipVehicBoatMed, "a-.-G-E-V-U-R-m"),
    (CoTType::GndEquipVehicBoatSmall, "a-.-G-E-V-U-R-s"),
    (CoTType::GndEquipVehicSemi, "a-.-G-E-V-U-S"),
    (CoTType::GndEquipVehicCrosscountrytruck, "a-.-G-E-V-U-X"),
    (CoTType::GndEquipVehicAmbulance, "a-.-G-E-V-m"),
    (CoTType::GndEquipWeapon, "a-.-G-E-W"),
    (CoTType::GndEquipWeaponAirdefensegun, "a-.-G-E-W-A"),
    (CoTType::GndEquipWeaponAaaHeavy, "a-.-G-E-W-A-H"),
    (CoTType::GndEquipWeaponAaaLight, "a-.-G-E-W-A-L"),
    (CoTType::GndEquipWeaponAaaMedium, "a-.-G-E-W-A-M"),
    (CoTType::GndEquipWeaponDirectfire, "a-.-G-E-W-D"),
    (CoTType::GndEquipWeaponDirectfireHeavy, "a-.-G-E-W-D-H"),
    (
        CoTType::GndEquipWeaponDirectfireHeavySelfpropelled,
        "a-.-G-E-W-D-H-S",
    ),
    (CoTType::GndEquipWeaponDirectfireLight, "a-.-G-E-W-D-L"),
    (
        CoTType::GndEquipWeaponDirectfireLightSelfpropelled,
        "a-.-G-E-W-D-L-S",
    ),
    (CoTType::GndEquipWeaponDirectfireMedium, "a-.-G-E-W-D-M"),
    (
        CoTType::GndEquipWeaponDirectfireMediumSelfpropelled,
        "a-.-G-E-W-D-M-S",
    ),
    (CoTType::GndEquipWeaponAnittankgun, "a-.-G-E-W-G"),
    (CoTType::GndEquipWeaponAtgHeavy, "a-.-G-E-W-G-H"),
    (CoTType::GndEquipWeaponAtgLight, "a-.-G-E-W-G-L"),
    (CoTType::GndEquipWeaponAtgMedium, "a-.-G-E-W-G-M"),
    (CoTType::GndEquipWeaponAtgRecoilless, "a-.-G-E-W-G-R"),
    (CoTType::GndEquipWeaponHowitzer, "a-.-G-E-W-H"),
    (CoTType::GndEquipWeaponHowitzerHeavy, "a-.-G-E-W-H-H"),
    (
        CoTType::GndEquipWeaponHowitzerHeavySelfpropelled,
        "a-.-G-E-W-H-H-S",
    ),
    (CoTType::GndEquipWeaponHowitzerLight, "a-.-G-E-W-H-L"),
    (
        CoTType::GndEquipWeaponHowitzerLightSelfpropelled,
        "a-.-G-E-W-H-L-S",
    ),
    (CoTType::GndEquipWeaponHowitzerMedium, "a-.-G-E-W-H-M"),
    (
        CoTType::GndEquipWeaponHowitzerMediumSelfpropelled,
        "a-.-G-E-W-H-M-S",
    ),
    (CoTType::GndEquipWeaponMissilelauncher, "a-.-G-E-W-M"),
    (
        CoTType::GndEquipWeaponMissilelauncherAdAirdefense,
        "a-.-G-E-W-M-A",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherAdIntermediaterange,
        "a-.-G-E-W-M-A-I",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherAdLongrange,
        "a-.-G-E-W-M-A-L",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherAdShortrange,
        "a-.-G-E-W-M-A-S",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherAdTheater,
        "a-.-G-E-W-M-A-T",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherSsSurfacesurfSs,
        "a-.-G-E-W-M-S",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherSsIntermediaterange,
        "a-.-G-E-W-M-S-I",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherSsLongrange,
        "a-.-G-E-W-M-S-L",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherSsShortrange,
        "a-.-G-E-W-M-S-S",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherAtAntitank,
        "a-.-G-E-W-M-T",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherAtHeavy,
        "a-.-G-E-W-M-T-H",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherAtLight,
        "a-.-G-E-W-M-T-L",
    ),
    (
        CoTType::GndEquipWeaponMissilelauncherAtMedium,
        "a-.-G-E-W-M-T-M",
    ),
    (CoTType::GndEquipWeaponMortar, "a-.-G-E-W-O"),
    (CoTType::GndEquipWeaponMortarHeavy, "a-.-G-E-W-O-H"),
    (CoTType::GndEquipWeaponMortarLight, "a-.-G-E-W-O-L"),
    (CoTType::GndEquipWeaponMortarMedium, "a-.-G-E-W-O-M"),
    (CoTType::GndEquipWeaponRifleAutomaticweapon, "a-.-G-E-W-R"),
    (CoTType::GndEquipWeaponRifleHeavymachinegun, "a-.-G-E-W-R-H"),
    (CoTType::GndEquipWeaponRifleLightmachinegun, "a-.-G-E-W-R-L"),
    (CoTType::GndEquipWeaponRifleRifle, "a-.-G-E-W-R-R"),
    (CoTType::GndEquipWeaponRocketlauncherSingle, "a-.-G-E-W-S"),
    (
        CoTType::GndEquipWeaponRocketlauncherSingleHeavy,
        "a-.-G-E-W-S-H",
    ),
    (
        CoTType::GndEquipWeaponRocketlauncherSingleLight,
        "a-.-G-E-W-S-L",
    ),
    (
        CoTType::GndEquipWeaponRocketlauncherSingleMedium,
        "a-.-G-E-W-S-M",
    ),
    (CoTType::GndEquipWeaponRocketlauncherAntitank, "a-.-G-E-W-T"),
    (
        CoTType::GndEquipWeaponRocketlauncherAtHeavy,
        "a-.-G-E-W-T-H",
    ),
    (
        CoTType::GndEquipWeaponRocketlauncherAtLight,
        "a-.-G-E-W-T-L",
    ),
    (
        CoTType::GndEquipWeaponRocketlauncherAtMedium,
        "a-.-G-E-W-T-M",
    ),
    (CoTType::GndEquipWeaponRocketlauncherMultiple, "a-.-G-E-W-X"),
    (
        CoTType::GndEquipWeaponRocketlauncherMultipleHeavy,
        "a-.-G-E-W-X-H",
    ),
    (
        CoTType::GndEquipWeaponRocketlauncherMultipleLight,
        "a-.-G-E-W-X-L",
    ),
    (
        CoTType::GndEquipWeaponRocketlauncherMultipleMedium,
        "a-.-G-E-W-X-M",
    ),
    (CoTType::GndEquipWeaponGrenadelauncher, "a-.-G-E-W-Z"),
    (CoTType::GndEquipWeaponGrenadelauncherHeavy, "a-.-G-E-W-Z-H"),
    (CoTType::GndEquipWeaponGrenadelauncherLight, "a-.-G-E-W-Z-L"),
    (
        CoTType::GndEquipWeaponGrenadelauncherMedium,
        "a-.-G-E-W-Z-M",
    ),
    (CoTType::GndEquipSpecialequipment, "a-.-G-E-X"),
    (CoTType::GndEquipFlamethrower, "a-.-G-E-X-F"),
    (CoTType::GndEquipLaser, "a-.-G-E-X-L"),
    (CoTType::GndEquipLandmines, "a-.-G-E-X-M"),
    (CoTType::GndEquipClaymore, "a-.-G-E-X-M-C"),
    (CoTType::GndEquipLessthanlethal, "a-.-G-E-X-M-L"),
    (CoTType::GndEquipNbcequipment, "a-.-G-E-X-N"),
    (CoTType::GndBuilding, "a-.-G-I"),
    (CoTType::GndStructureBaseMilitary, "a-.-G-I-B"),
    (CoTType::GndStructureBaseAirbase, "a-.-G-I-B-A"),
    (CoTType::GndStructureBaseNavalbase, "a-.-G-I-B-N"),
    (CoTType::GndStructureCivilian, "a-.-G-I-c"),
    (CoTType::GndStructureCivilianBridge, "a-.-G-I-c-b"),
    (
        CoTType::GndStructureCivilianFenceorwallorbarrier,
        "a-.-G-I-c-bar",
    ),
    (CoTType::GndStructureCivilianCanal, "a-.-G-I-c-can"),
    (CoTType::GndStructureCivilianCanalLock, "a-.-G-I-c-can-l"),
    (CoTType::GndStructureCivilianFarm, "a-.-G-I-c-frm"),
    (CoTType::GndStructureCivilianLevee, "a-.-G-I-c-l"),
    (CoTType::GndStructureCivilianMonument, "a-.-G-I-c-mon"),
    (CoTType::GndStructureCivilianOffices, "a-.-G-I-c-o"),
    (
        CoTType::GndStructureCivilianResidenceorapartmentorhotel,
        "a-.-G-I-c-rah",
    ),
    (CoTType::GndStructureCivilianReligous, "a-.-G-I-c-rel"),
    (CoTType::GndStructureCivilianResevoir, "a-.-G-I-c-res"),
    (CoTType::GndStructureCivilianRetail, "a-.-G-I-c-ret"),
    (CoTType::GndStructureCivilianSchool, "a-.-G-I-c-sch"),
    (CoTType::GndStructureCivilianVip, "a-.-G-I-c-vip"),
    (CoTType::GndStructureCivilianWarehouse, "a-.-G-I-c-whs"),
    (CoTType::GndStructureFactory, "a-.-G-I-E"),
    (CoTType::GndStructureFactoryHeavy, "a-.-G-I-E-h"),
    (CoTType::GndStructureFactoryLight, "a-.-G-I-E-l"),
    (CoTType::GndStructureFactoryPlantother, "a-.-G-I-E-o"),
    (CoTType::GndStructureGovernment, "a-.-G-I-G"),
    (CoTType::GndStructureImfacilities, "a-.-G-I-i"),
    (
        CoTType::GndStructureImfacilitiesEmergencymanagement,
        "a-.-G-I-i-e",
    ),
    (
        CoTType::GndStructureImfacilitiesEmergencymanagementEmergencyoperationscenter,
        "a-.-G-I-i-e-eoc",
    ),
    (
        CoTType::GndStructureImfacilitiesEmergencymanagementIncidentcommander,
        "a-.-G-I-i-e-ic",
    ),
    (
        CoTType::GndStructureImfacilitiesEmergencymanagementIncidentcommunicationscenter,
        "a-.-G-I-i-e-icc",
    ),
    (
        CoTType::GndStructureImfacilitiesEmergencymanagementIncidentcommandpost,
        "a-.-G-I-i-e-icp",
    ),
    (
        CoTType::GndStructureImfacilitiesEmergencymanagementWarroom,
        "a-.-G-I-i-e-wr",
    ),
    (
        CoTType::GndStructureImfacilitiesFireandhazmat,
        "a-.-G-I-i-f",
    ),
    (
        CoTType::GndStructureImfacilitiesFireandhazmatFiredepartment,
        "a-.-G-I-i-f-fd",
    ),
    (
        CoTType::GndStructureImfacilitiesHealthandmedical,
        "a-.-G-I-i-h",
    ),
    (
        CoTType::GndStructureImfacilitiesHealthandmedicalMedicaldispatch,
        "a-.-G-I-i-h-md",
    ),
    (
        CoTType::GndStructureImfacilitiesLawenforcement,
        "a-.-G-I-i-l",
    ),
    (
        CoTType::GndStructureImfacilitiesLawenforcementPolicecheckpoint,
        "a-.-G-I-i-l-cp",
    ),
    (
        CoTType::GndStructureImfacilitiesLawenforcementHighwaypatrol,
        "a-.-G-I-i-l-hp",
    ),
    (
        CoTType::GndStructureImfacilitiesLawenforcementLocalpolice,
        "a-.-G-I-i-l-lp",
    ),
    (
        CoTType::GndStructureImfacilitiesLawenforcementNotrespassingzone,
        "a-.-G-I-i-l-ntz",
    ),
    (
        CoTType::GndStructureImfacilitiesLawenforcementSheriffsdepartment,
        "a-.-G-I-i-l-sd",
    ),
    (
        CoTType::GndStructureImfacilitiesLawenforcementStatepolice,
        "a-.-G-I-i-l-sp",
    ),
    (
        CoTType::GndStructureImfacilitiesEmergencymedicalservices,
        "a-.-G-I-i-m",
    ),
    (CoTType::GndStructureImfacilitiesOther, "a-.-G-I-i-o"),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatch,
        "a-.-G-I-i-o-ad",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatchEms,
        "a-.-G-I-i-o-ad-ems",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatchFire,
        "a-.-G-I-i-o-ad-f",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatchHighway,
        "a-.-G-I-i-o-ad-hwy",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatchMedia,
        "a-.-G-I-i-o-ad-mda",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatchPolicelocal,
        "a-.-G-I-i-o-ad-pl",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatchPolicestation,
        "a-.-G-I-i-o-ad-ps",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatchTow,
        "a-.-G-I-i-o-ad-tow",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherAgencydispatchTransit,
        "a-.-G-I-i-o-ad-ts",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherBordercontrolcheckpoint,
        "a-.-G-I-i-o-bcc",
    ),
    (CoTType::GndStructureImfacilitiesOtherDmz, "a-.-G-I-i-o-dmz"),
    (
        CoTType::GndStructureImfacilitiesOtherIndependentserviceprovider,
        "a-.-G-I-i-o-isp",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilecenter,
        "a-.-G-I-i-o-mc",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilecenterFire,
        "a-.-G-I-i-o-mc-f",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilecenterMedical,
        "a-.-G-I-i-o-mc-h",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilecenterLawenforcement,
        "a-.-G-I-i-o-mc-l",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMessagecenter,
        "a-.-G-I-i-o-mes",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenter,
        "a-.-G-I-i-o-mob",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenterEms,
        "a-.-G-I-i-o-mob-ems",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenterTransit,
        "a-.-G-I-i-o-mob-es",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenterFire,
        "a-.-G-I-i-o-mob-f",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenterHighway,
        "a-.-G-I-i-o-mob-hwy",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenterMedia,
        "a-.-G-I-i-o-mob-mda",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenterPolicelocal,
        "a-.-G-I-i-o-mob-pl",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenterPolicestation,
        "a-.-G-I-i-o-mob-ps",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherMobilizationcenterTow,
        "a-.-G-I-i-o-mob-tow",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherPublicsafety,
        "a-.-G-I-i-o-ps",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherReportinglocations,
        "a-.-G-I-i-o-rl",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingarea,
        "a-.-G-I-i-o-sa",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingareaEms,
        "a-.-G-I-i-o-sa-ems",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingareaFire,
        "a-.-G-I-i-o-sa-f",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingareaHighway,
        "a-.-G-I-i-o-sa-hwy",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingareaMedia,
        "a-.-G-I-i-o-sa-mda",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingareaPolicelocal,
        "a-.-G-I-i-o-sa-pl",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingareaPolicestation,
        "a-.-G-I-i-o-sa-ps",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingareaTow,
        "a-.-G-I-i-o-sa-tow",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherStagingareaTransit,
        "a-.-G-I-i-o-sa-ts",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherSecuritycheckpoint,
        "a-.-G-I-i-o-sc",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherSegment,
        "a-.-G-I-i-o-seg",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTransportation,
        "a-.-G-I-i-o-t",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTransportationTransitincidentmanagementcenter,
        "a-.-G-I-i-o-t-im",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTransportationTransitmanagementcenter,
        "a-.-G-I-i-o-t-tc",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTransportationTrafficincservicepatroldispatch,
        "a-.-G-I-i-o-t-td",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTransportationTrafficmanagementcenter,
        "a-.-G-I-i-o-t-tm",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTemp,
        "a-.-G-I-i-o-tmp",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTempFire,
        "a-.-G-I-i-o-tmp-f",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTempMedical,
        "a-.-G-I-i-o-tmp-h",
    ),
    (
        CoTType::GndStructureImfacilitiesOtherTempLawenforcement,
        "a-.-G-I-i-o-tmp-l",
    ),
    (CoTType::GndStructureImfacilitiesPublicworks, "a-.-G-I-i-p"),
    (
        CoTType::GndStructureMilMilitarymaterielfacility,
        "a-.-G-I-M",
    ),
    (CoTType::GndStructureMilAircraftproduction, "a-.-G-I-M-A"),
    (CoTType::GndStructureMilNbcwarfareproduction, "a-.-G-I-M-C"),
    (CoTType::GndStructureMilAmmunitionproduction, "a-.-G-I-M-E"),
    (CoTType::GndStructureMilNuclearenergy, "a-.-G-I-M-F"),
    (CoTType::GndStructureMilAtomicenergyreactor, "a-.-G-I-M-F-A"),
    (
        CoTType::GndStructureMilNuclearmaterialproduction,
        "a-.-G-I-M-F-P",
    ),
    (
        CoTType::GndStructureMilNuclearWeaponsgrade,
        "a-.-G-I-M-F-P-W",
    ),
    (
        CoTType::GndStructureMilNuclearmaterialstorage,
        "a-.-G-I-M-F-S",
    ),
    (CoTType::GndStructureMilArmamentproduction, "a-.-G-I-M-G"),
    (
        CoTType::GndStructureMilMissileSpacesystemproduction,
        "a-.-G-I-M-M",
    ),
    (
        CoTType::GndStructureMilEngineeringequipmentproduction,
        "a-.-G-I-M-N",
    ),
    (CoTType::GndStructureMilBridge, "a-.-G-I-M-N-B"),
    (CoTType::GndStructureMilBridgeLarge, "a-.-G-I-M-N-B-l"),
    (CoTType::GndStructureMilBridgeMed, "a-.-G-I-M-N-B-m"),
    (CoTType::GndStructureMilBridgeSmall, "a-.-G-I-M-N-B-s"),
    (CoTType::GndStructureMilCanal, "a-.-G-I-M-N-c"),
    (CoTType::GndStructureMilShipconstruction, "a-.-G-I-M-S"),
    (
        CoTType::GndStructureMilMilitaryvehicleproduction,
        "a-.-G-I-M-V",
    ),
    (CoTType::GndStructureProcessingfacility, "a-.-G-I-P"),
    (CoTType::GndStructureDecon, "a-.-G-I-P-D"),
    (
        CoTType::GndStructureRawmaterialproductionStorage,
        "a-.-G-I-R",
    ),
    (CoTType::GndStructureMine, "a-.-G-I-R-M"),
    (CoTType::GndStructureNbc, "a-.-G-I-R-N"),
    (CoTType::GndStructureBiological, "a-.-G-I-R-N-B"),
    (CoTType::GndStructureChemical, "a-.-G-I-R-N-C"),
    (CoTType::GndStructureNuclear, "a-.-G-I-R-N-N"),
    (CoTType::GndStructurePetroleumGasOil, "a-.-G-I-R-P"),
    (
        CoTType::GndStructurePetroleumGasOilRefinery,
        "a-.-G-I-R-P-r",
    ),
    (CoTType::GndStructureTransportfacility, "a-.-G-I-T"),
    (CoTType::GndStructureTransportfacilityAirport, "a-.-G-I-T-a"),
    (
        CoTType::GndStructureTransportfacilityHelibase,
        "a-.-G-I-T-hb",
    ),
    (
        CoTType::GndStructureTransportfacilityHelispot,
        "a-.-G-I-T-hs",
    ),
    (
        CoTType::GndStructureTransportfacilityLandport,
        "a-.-G-I-T-l",
    ),
    (
        CoTType::GndStructureTransportfacilityParkinggarage,
        "a-.-G-I-T-pg",
    ),
    (
        CoTType::GndStructureTransportfacilityRailroad,
        "a-.-G-I-T-r",
    ),
    (CoTType::GndStructureTransportfacilitySeaport, "a-.-G-I-T-s"),
    (
        CoTType::GndStructureUtilityServiceResearchUtilityfacility,
        "a-.-G-I-U",
    ),
    (CoTType::GndStructureUtilityElectricpower, "a-.-G-I-U-E"),
    (
        CoTType::GndStructureUtilityElectricpowerCoalpowerplant,
        "a-.-G-I-U-E-c",
    ),
    (
        CoTType::GndStructureUtilityElectricpowerHydroelectricpowerplant,
        "a-.-G-I-U-E-h",
    ),
    (
        CoTType::GndStructureUtilityElectricpowerOtherpower,
        "a-.-G-I-U-E-o",
    ),
    (
        CoTType::GndStructureUtilityElectricpowerPowersubstation,
        "a-.-G-I-U-E-ps",
    ),
    (CoTType::GndStructureUtilityDam, "a-.-G-I-U-E-D"),
    (CoTType::GndStructureUtilityFossilfuel, "a-.-G-I-U-E-F"),
    (CoTType::GndStructureUtilityNuclearplant, "a-.-G-I-U-E-N"),
    (
        CoTType::GndStructureUtilityPublicwaterservices,
        "a-.-G-I-U-P",
    ),
    (
        CoTType::GndStructureUtilityTechnologicalresearch,
        "a-.-G-I-U-R",
    ),
    (
        CoTType::GndStructureUtilityTelecommunications,
        "a-.-G-I-U-T",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsCommunications,
        "a-.-G-I-U-T-com",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsCommunicationsAntennafarm,
        "a-.-G-I-U-T-com-af",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsCommunicationsSatellitecommunications,
        "a-.-G-I-U-T-com-sat",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsCommunicationsTower,
        "a-.-G-I-U-T-com-tow",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsRadio,
        "a-.-G-I-U-T-r",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsRadioStation,
        "a-.-G-I-U-T-r-s",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsTelephone,
        "a-.-G-I-U-T-tp",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsTelephoneTelephoneswitching,
        "a-.-G-I-U-T-tp-ts",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsTelevision,
        "a-.-G-I-U-T-tv",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsTelevisionCable,
        "a-.-G-I-U-T-tv-c",
    ),
    (
        CoTType::GndStructureUtilityTelecommunicationsTelevisionStation,
        "a-.-G-I-U-T-tv-s",
    ),
    (CoTType::GndStructureMedicalfacility, "a-.-G-I-X"),
    (CoTType::GndStructureHospital, "a-.-G-I-X-H"),
    (
        CoTType::GndStructureMedicalfacilityHospitalHospitalcarefacility,
        "a-.-G-I-X-hcf",
    ),
    (CoTType::GndStructureRoad, "a-.-G-I-r"),
    (CoTType::GndStructureRoadHighway, "a-.-G-I-r-h"),
    (CoTType::GndStructureRoadImproved, "a-.-G-I-r-i"),
    (CoTType::GndStructureRoadRestarea, "a-.-G-I-r-ra"),
    (CoTType::GndStructureRoadUnimproved, "a-.-G-I-r-u"),
    (CoTType::GndUnit, "a-.-G-U"),
    (CoTType::GndCombat, "a-.-G-U-C"),
    (CoTType::GndCombatArmor, "a-.-G-U-C-A"),
    (CoTType::GndCombatArmorAntiarmor, "a-.-G-U-C-A-A"),
    (CoTType::GndCombatArmorAntiarmorarmored, "a-.-G-U-C-A-A-A"),
    (
        CoTType::GndCombatArmorAntiarmorarmoredairassault,
        "a-.-G-U-C-A-A-A-S",
    ),
    (
        CoTType::GndCombatArmorAntiarmorarmoredtracked,
        "a-.-G-U-C-A-A-A-T",
    ),
    (
        CoTType::GndCombatArmorAntiarmorarmoredwheeled,
        "a-.-G-U-C-A-A-A-W",
    ),
    (CoTType::GndCombatArmorAntiarmorarctic, "a-.-G-U-C-A-A-C"),
    (
        CoTType::GndCombatArmorAntiarmordismounted,
        "a-.-G-U-C-A-A-D",
    ),
    (CoTType::GndCombatArmorAntiarmorlight, "a-.-G-U-C-A-A-L"),
    (CoTType::GndCombatArmorAntiarmorairborne, "a-.-G-U-C-A-A-M"),
    (CoTType::GndCombatArmorAntiarmormotorized, "a-.-G-U-C-A-A-O"),
    (
        CoTType::GndCombatArmorAntiarmormotorizedairassault,
        "a-.-G-U-C-A-A-O-S",
    ),
    (
        CoTType::GndCombatArmorAntiarmorairassault,
        "a-.-G-U-C-A-A-S",
    ),
    (CoTType::GndCombatArmorAntiarmormountain, "a-.-G-U-C-A-A-U"),
    (CoTType::GndCombatArmorArmortrack, "a-.-G-U-C-A-T"),
    (CoTType::GndCombatArmorArmortrackairborne, "a-.-G-U-C-A-T-A"),
    (CoTType::GndCombatArmorArmortrackHeavy, "a-.-G-U-C-A-T-H"),
    (CoTType::GndCombatArmorArmortrackLight, "a-.-G-U-C-A-T-L"),
    (CoTType::GndCombatArmorArmortrackMedium, "a-.-G-U-C-A-T-M"),
    (CoTType::GndCombatArmorArmortrackRecovery, "a-.-G-U-C-A-T-R"),
    (
        CoTType::GndCombatArmorArmortrackamphibious,
        "a-.-G-U-C-A-T-W",
    ),
    (
        CoTType::GndCombatArmorArmortrackamphibiousrecovery,
        "a-.-G-U-C-A-T-W-R",
    ),
    (CoTType::GndCombatArmorArmorWheeled, "a-.-G-U-C-A-W"),
    (
        CoTType::GndCombatArmorArmorWheeledairborne,
        "a-.-G-U-C-A-W-A",
    ),
    (CoTType::GndCombatArmorArmorWheeledheavy, "a-.-G-U-C-A-W-H"),
    (CoTType::GndCombatArmorArmorWheeledlight, "a-.-G-U-C-A-W-L"),
    (CoTType::GndCombatArmorArmorWheeledmedium, "a-.-G-U-C-A-W-M"),
    (
        CoTType::GndCombatArmorArmorWheeledrecovery,
        "a-.-G-U-C-A-W-R",
    ),
    (
        CoTType::GndCombatArmorArmorWheeledairassault,
        "a-.-G-U-C-A-W-S",
    ),
    (
        CoTType::GndCombatArmorArmorWheeledamphibious,
        "a-.-G-U-C-A-W-W",
    ),
    (
        CoTType::GndCombatArmorArmorWheeledamphibiousrecovery,
        "a-.-G-U-C-A-W-W-R",
    ),
    (CoTType::GndCombatDefenseAirdefense, "a-.-G-U-C-D"),
    (CoTType::GndCombatDefenseComposite, "a-.-G-U-C-D-C"),
    (CoTType::GndCombatDefenseGununit, "a-.-G-U-C-D-G"),
    (CoTType::GndCombatDefenseHMad, "a-.-G-U-C-D-H"),
    (CoTType::GndCombatDefenseHawk, "a-.-G-U-C-D-H-H"),
    (CoTType::GndCombatDefensePatriot, "a-.-G-U-C-D-H-P"),
    (CoTType::GndCombatDefenseAirdefensemissile, "a-.-G-U-C-D-M"),
    (
        CoTType::GndCombatDefenseAirdefensemissileheavy,
        "a-.-G-U-C-D-M-H",
    ),
    (
        CoTType::GndCombatDefenseAirdefensemissilelight,
        "a-.-G-U-C-D-M-L",
    ),
    (
        CoTType::GndCombatDefenseAirdefensemissilemotorizedAvenger,
        "a-.-G-U-C-D-M-L-A",
    ),
    (
        CoTType::GndCombatDefenseAirdefensemissilemedium,
        "a-.-G-U-C-D-M-M",
    ),
    (
        CoTType::GndCombatDefenseTheatermissiledefenseunit,
        "a-.-G-U-C-D-O",
    ),
    (CoTType::GndCombatDefenseShortrange, "a-.-G-U-C-D-S"),
    (CoTType::GndCombatDefenseStinger, "a-.-G-U-C-D-S-S"),
    (CoTType::GndCombatDefenseVulcan, "a-.-G-U-C-D-S-V"),
    (CoTType::GndCombatDefenseTargetingunit, "a-.-G-U-C-D-T"),
    (CoTType::GndCombatEngineerEngineer, "a-.-G-U-C-E"),
    (CoTType::GndCombatEngineerEngineercombat, "a-.-G-U-C-E-C"),
    (
        CoTType::GndCombatEngineerEngineercombatairborne,
        "a-.-G-U-C-E-C-A",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatarctic,
        "a-.-G-U-C-E-C-C",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatheavy,
        "a-.-G-U-C-E-C-H",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatlightSapper,
        "a-.-G-U-C-E-C-L",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatmedium,
        "a-.-G-U-C-E-C-M",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatmountain,
        "a-.-G-U-C-E-C-O",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatrecon,
        "a-.-G-U-C-E-C-R",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatairassault,
        "a-.-G-U-C-E-C-S",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatmechanizedTrack,
        "a-.-G-U-C-E-C-T",
    ),
    (
        CoTType::GndCombatEngineerEngineercombatmotorized,
        "a-.-G-U-C-E-C-W",
    ),
    (
        CoTType::GndCombatEngineerEngineerconstruction,
        "a-.-G-U-C-E-N",
    ),
    (
        CoTType::GndCombatEngineerEngineernavalconstruction,
        "a-.-G-U-C-E-N-N",
    ),
    (CoTType::GndCombatArtilleryFixed, "a-.-G-U-C-F"),
    (CoTType::GndCombatHowitzerGun, "a-.-G-U-C-F-H"),
    (CoTType::GndCombatAirborne, "a-.-G-U-C-F-H-A"),
    (CoTType::GndCombatArctic, "a-.-G-U-C-F-H-C"),
    (CoTType::GndCombatArtilleryMobile, "a-.-G-U-C-F-H-E"),
    (CoTType::GndCombatHeavy, "a-.-G-U-C-F-H-H"),
    (CoTType::GndCombatLight, "a-.-G-U-C-F-H-L"),
    (CoTType::GndCombatMedium, "a-.-G-U-C-F-H-M"),
    (CoTType::GndCombatMountain, "a-.-G-U-C-F-H-O"),
    (CoTType::GndCombatAirassault, "a-.-G-U-C-F-H-S"),
    (CoTType::GndCombatAmphibious, "a-.-G-U-C-F-H-X"),
    (CoTType::GndCombatMortar, "a-.-G-U-C-F-M"),
    (CoTType::GndCombatAmphibiousmortar, "a-.-G-U-C-F-M-L"),
    (
        CoTType::GndCombatSelfpropelledSpTrackedmortar,
        "a-.-G-U-C-F-M-S",
    ),
    (CoTType::GndCombatTowedmortar, "a-.-G-U-C-F-M-T"),
    (CoTType::GndCombatTowedairbornemortar, "a-.-G-U-C-F-M-T-A"),
    (CoTType::GndCombatTowedarcticmortar, "a-.-G-U-C-F-M-T-C"),
    (CoTType::GndCombatTowedmountainmortar, "a-.-G-U-C-F-M-T-O"),
    (CoTType::GndCombatTowedairassaultmortar, "a-.-G-U-C-F-M-T-S"),
    (CoTType::GndCombatSpwheeledmortar, "a-.-G-U-C-F-M-W"),
    (CoTType::GndCombatMeteorological, "a-.-G-U-C-F-O"),
    (CoTType::GndCombatAirbornemeteorological, "a-.-G-U-C-F-O-A"),
    (CoTType::GndCombatLightmeteorological, "a-.-G-U-C-F-O-L"),
    (CoTType::GndCombatMountainmeteorological, "a-.-G-U-C-F-O-O"),
    (
        CoTType::GndCombatAirassaultmeteorological,
        "a-.-G-U-C-F-O-S",
    ),
    (CoTType::GndCombatRocket, "a-.-G-U-C-F-R"),
    (CoTType::GndCombatRocketsFixed, "a-.-G-U-C-F-R-M"),
    (CoTType::GndCombatRocketsMobile, "a-.-G-U-C-F-R-M-R"),
    (
        CoTType::GndCombatMultirocketselfpropelled,
        "a-.-G-U-C-F-R-M-S",
    ),
    (CoTType::GndCombatMultirockettowed, "a-.-G-U-C-F-R-M-T"),
    (CoTType::GndCombatSinglerocketlauncher, "a-.-G-U-C-F-R-S"),
    (CoTType::GndCombatSinglerockettruck, "a-.-G-U-C-F-R-S-R"),
    (
        CoTType::GndCombatSinglerocketselfpropelled,
        "a-.-G-U-C-F-R-S-S",
    ),
    (CoTType::GndCombatSinglerockettowed, "a-.-G-U-C-F-R-S-T"),
    (CoTType::GndCombatArtillerysurvey, "a-.-G-U-C-F-S"),
    (CoTType::GndCombatAirborne, "a-.-G-U-C-F-S-A"),
    (CoTType::GndCombatLight, "a-.-G-U-C-F-S-L"),
    (CoTType::GndCombatMountain, "a-.-G-U-C-F-S-O"),
    (CoTType::GndCombatAirassault, "a-.-G-U-C-F-S-S"),
    (CoTType::GndCombatTargetacquisition, "a-.-G-U-C-F-T"),
    (CoTType::GndCombatAnglico, "a-.-G-U-C-F-T-A"),
    (CoTType::GndCombatColtFist, "a-.-G-U-C-F-T-C"),
    (CoTType::GndCombatDismountedcoltFist, "a-.-G-U-C-F-T-C-D"),
    (CoTType::GndCombatTrackedcoltFist, "a-.-G-U-C-F-T-C-M"),
    (CoTType::GndCombatFlashOptical, "a-.-G-U-C-F-T-F"),
    (CoTType::GndCombatRadar, "a-.-G-U-C-F-T-R"),
    (CoTType::GndCombatSound, "a-.-G-U-C-F-T-S"),
    (CoTType::GndCombatInfantryTroopsOpen, "a-.-G-U-C-I"),
    (CoTType::GndCombatInfantryAirborne, "a-.-G-U-C-I-A"),
    (CoTType::GndCombatInfantryArctic, "a-.-G-U-C-I-C"),
    (CoTType::GndCombatInfantryFightingvehicle, "a-.-G-U-C-I-I"),
    (CoTType::GndCombatInfantryLight, "a-.-G-U-C-I-L"),
    (CoTType::GndCombatInfantryMotorized, "a-.-G-U-C-I-M"),
    (CoTType::GndCombatInfantryNaval, "a-.-G-U-C-I-N"),
    (CoTType::GndCombatInfantryMountain, "a-.-G-U-C-I-O"),
    (CoTType::GndCombatInfantryAirassault, "a-.-G-U-C-I-S"),
    (CoTType::GndCombatInfantryMechanized, "a-.-G-U-C-I-Z"),
    (CoTType::GndCombatInfantryDugin, "a-.-G-U-C-I-d"),
    (CoTType::GndCombatMissileSurfsurf, "a-.-G-U-C-M"),
    (CoTType::GndCombatMissileSurfsurfStrategic, "a-.-G-U-C-M-S"),
    (CoTType::GndCombatMissileSurfsurfTactical, "a-.-G-U-C-M-T"),
    (CoTType::GndCombatRecon, "a-.-G-U-C-R"),
    (CoTType::GndCombatReconAirborne, "a-.-G-U-C-R-A"),
    (CoTType::GndCombatReconArctic, "a-.-G-U-C-R-C"),
    (CoTType::GndCombatReconHorse, "a-.-G-U-C-R-H"),
    (CoTType::GndCombatReconLight, "a-.-G-U-C-R-L"),
    (CoTType::GndCombatReconMountain, "a-.-G-U-C-R-O"),
    (CoTType::GndCombatReconMarine, "a-.-G-U-C-R-R"),
    (CoTType::GndCombatReconMarinedivision, "a-.-G-U-C-R-R-D"),
    (CoTType::GndCombatReconMarineforce, "a-.-G-U-C-R-R-F"),
    (CoTType::GndCombatReconMarinelightarmored, "a-.-G-U-C-R-R-L"),
    (CoTType::GndCombatReconAirassault, "a-.-G-U-C-R-S"),
    (CoTType::GndCombatReconCavalry, "a-.-G-U-C-R-V"),
    (CoTType::GndCombatReconCavalryarmored, "a-.-G-U-C-R-V-A"),
    (CoTType::GndCombatReconCavalryground, "a-.-G-U-C-R-V-G"),
    (CoTType::GndCombatReconCavalrymotorized, "a-.-G-U-C-R-V-M"),
    (CoTType::GndCombatReconCavalryair, "a-.-G-U-C-R-V-O"),
    (
        CoTType::GndCombatReconLongrangesurveillance,
        "a-.-G-U-C-R-X",
    ),
    (
        CoTType::GndCombatSecurityInternalsecurityforces,
        "a-.-G-U-C-S",
    ),
    (CoTType::GndCombatSecurityAviation, "a-.-G-U-C-S-A"),
    (CoTType::GndCombatSecurityGround, "a-.-G-U-C-S-G"),
    (
        CoTType::GndCombatSecurityMechanizedground,
        "a-.-G-U-C-S-G-A",
    ),
    (
        CoTType::GndCombatSecurityDismountedground,
        "a-.-G-U-C-S-G-D",
    ),
    (CoTType::GndCombatSecurityMotorizedground, "a-.-G-U-C-S-G-M"),
    (CoTType::GndCombatSecurityWheeledmechanized, "a-.-G-U-C-S-M"),
    (CoTType::GndCombatSecurityRail, "a-.-G-U-C-S-R"),
    (CoTType::GndCombatSecurityRiverine, "a-.-G-U-C-S-W"),
    (CoTType::GndCombatAviationAviation, "a-.-G-U-C-V"),
    (CoTType::GndCombatAviationComposite, "a-.-G-U-C-V-C"),
    (CoTType::GndCombatAviationFixedwing, "a-.-G-U-C-V-F"),
    (CoTType::GndCombatAviationAttackfixedwing, "a-.-G-U-C-V-F-A"),
    (CoTType::GndCombatAviationReconfixedwing, "a-.-G-U-C-V-F-R"),
    (
        CoTType::GndCombatAviationUtilityfixedwing,
        "a-.-G-U-C-V-F-U",
    ),
    (CoTType::GndCombatAviationRotarywing, "a-.-G-U-C-V-R"),
    (
        CoTType::GndCombatAviationAttackrotarywing,
        "a-.-G-U-C-V-R-A",
    ),
    (
        CoTType::GndCombatAviationMinecountermeasurerotarywing,
        "a-.-G-U-C-V-R-M",
    ),
    (CoTType::GndCombatAviationScoutrotarywing, "a-.-G-U-C-V-R-S"),
    (
        CoTType::GndCombatAviationUtilityrotarywing,
        "a-.-G-U-C-V-R-U",
    ),
    (CoTType::GndCombatAviationC2Rotarywing, "a-.-G-U-C-V-R-U-C"),
    (
        CoTType::GndCombatAviationMedevacrotarywing,
        "a-.-G-U-C-V-R-U-E",
    ),
    (
        CoTType::GndCombatAviationHeavyutilityrotarywing,
        "a-.-G-U-C-V-R-U-H",
    ),
    (
        CoTType::GndCombatAviationLightutilityrotarywing,
        "a-.-G-U-C-V-R-U-L",
    ),
    (
        CoTType::GndCombatAviationMediumutilityrotarywing,
        "a-.-G-U-C-V-R-U-M",
    ),
    (
        CoTType::GndCombatAviationAntisubmarinewarfarerotarywing,
        "a-.-G-U-C-V-R-W",
    ),
    (CoTType::GndCombatAviationHeloDual, "a-.-G-U-C-V-R-d"),
    (CoTType::GndCombatAviationHeloSingle, "a-.-G-U-C-V-R-s"),
    (CoTType::GndCombatAviationSearchandrescue, "a-.-G-U-C-V-S"),
    (
        CoTType::GndCombatAviationUnmannedaerialvehicle,
        "a-.-G-U-C-V-U",
    ),
    (
        CoTType::GndCombatAviationUnmannedaerialvehiclefixedwing,
        "a-.-G-U-C-V-U-F",
    ),
    (
        CoTType::GndCombatAviationUnmannedaerialvehiclerotarywing,
        "a-.-G-U-C-V-U-R",
    ),
    (
        CoTType::GndCombatAviationVerticalShorttakeoffandlanding,
        "a-.-G-U-C-V-V",
    ),
    (CoTType::GndSpecialc2Headquarterscomponent, "a-.-G-U-H"),
    (CoTType::GndCombatservicesupport, "a-.-G-U-S"),
    (CoTType::GndAdministrativeAdmin, "a-.-G-U-S-A"),
    (CoTType::GndAdmincorps, "a-.-G-U-S-A-C"),
    (CoTType::GndFinance, "a-.-G-U-S-A-F"),
    (CoTType::GndFinancecorps, "a-.-G-U-S-A-F-C"),
    (CoTType::GndFinancetheater, "a-.-G-U-S-A-F-T"),
    (CoTType::GndJudgeadvocategeneralJag, "a-.-G-U-S-A-J"),
    (CoTType::GndJagcorps, "a-.-G-U-S-A-J-C"),
    (CoTType::GndJagtheater, "a-.-G-U-S-A-J-T"),
    (CoTType::GndLabor, "a-.-G-U-S-A-L"),
    (CoTType::GndLaborcorps, "a-.-G-U-S-A-L-C"),
    (CoTType::GndLabortheater, "a-.-G-U-S-A-L-T"),
    (CoTType::GndMortuaryGravesregistry, "a-.-G-U-S-A-M"),
    (CoTType::GndMortuaryGravesregistrycorps, "a-.-G-U-S-A-M-C"),
    (CoTType::GndMortuaryGravesregistrytheater, "a-.-G-U-S-A-M-T"),
    (CoTType::GndPostal, "a-.-G-U-S-A-O"),
    (CoTType::GndPostalcorps, "a-.-G-U-S-A-O-C"),
    (CoTType::GndPostaltheater, "a-.-G-U-S-A-O-T"),
    (CoTType::GndPublicaffairs, "a-.-G-U-S-A-P"),
    (CoTType::GndPublicaffairsbroadcast, "a-.-G-U-S-A-P-B"),
    (CoTType::GndPublicaffairsbroadcastcorps, "a-.-G-U-S-A-P-B-C"),
    (
        CoTType::GndPublicaffairsbroadcasttheater,
        "a-.-G-U-S-A-P-B-T",
    ),
    (CoTType::GndPublicaffairscorps, "a-.-G-U-S-A-P-C"),
    (
        CoTType::GndPublicaffairsjointinformationbureau,
        "a-.-G-U-S-A-P-M",
    ),
    (CoTType::GndPublicaffairsjibcorps, "a-.-G-U-S-A-P-M-C"),
    (CoTType::GndPublicaffairsjibtheater, "a-.-G-U-S-A-P-M-T"),
    (CoTType::GndPublicaffairstheater, "a-.-G-U-S-A-P-T"),
    (CoTType::GndQuartermasterSupply, "a-.-G-U-S-A-Q"),
    (CoTType::GndQuartermasterSupplyCorps, "a-.-G-U-S-A-Q-C"),
    (CoTType::GndQuartermasterSupplyTheater, "a-.-G-U-S-A-Q-T"),
    (CoTType::GndReligiousChaplain, "a-.-G-U-S-A-R"),
    (CoTType::GndReligiousChaplaincorps, "a-.-G-U-S-A-R-C"),
    (CoTType::GndReligiousChaplaintheater, "a-.-G-U-S-A-R-T"),
    (CoTType::GndPersonnelservices, "a-.-G-U-S-A-S"),
    (CoTType::GndPersonnelcorps, "a-.-G-U-S-A-S-C"),
    (CoTType::GndPersonneltheater, "a-.-G-U-S-A-S-T"),
    (CoTType::GndAdmintheater, "a-.-G-U-S-A-T"),
    (CoTType::GndMoraleWelfareRecreationMwr, "a-.-G-U-S-A-W"),
    (CoTType::GndMwrcorps, "a-.-G-U-S-A-W-C"),
    (CoTType::GndMwrtheater, "a-.-G-U-S-A-W-T"),
    (CoTType::GndReplacementholdingunitRhu, "a-.-G-U-S-A-X"),
    (CoTType::GndRhucorps, "a-.-G-U-S-A-X-C"),
    (CoTType::GndRhutheater, "a-.-G-U-S-A-X-T"),
    (CoTType::GndMedical, "a-.-G-U-S-M"),
    (CoTType::GndMedicalcorps, "a-.-G-U-S-M-C"),
    (CoTType::GndMedicaldental, "a-.-G-U-S-M-D"),
    (CoTType::GndMedicaldentalcorps, "a-.-G-U-S-M-D-C"),
    (CoTType::GndMedicaldentaltheater, "a-.-G-U-S-M-D-T"),
    (CoTType::GndMedicaltreatmentfacility, "a-.-G-U-S-M-M"),
    (CoTType::GndMedicaltreatmentfacilitycorps, "a-.-G-U-S-M-M-C"),
    (
        CoTType::GndMedicaltreatmentfacilitytheater,
        "a-.-G-U-S-M-M-T",
    ),
    (CoTType::GndMedicalpsychological, "a-.-G-U-S-M-P"),
    (CoTType::GndMedicalpsychologicalcorps, "a-.-G-U-S-M-P-C"),
    (CoTType::GndMedicalpsychologicaltheater, "a-.-G-U-S-M-P-T"),
    (CoTType::GndMedicaltheater, "a-.-G-U-S-M-T"),
    (CoTType::GndMedicalveterinary, "a-.-G-U-S-M-V"),
    (CoTType::GndMedicalveterinarycorps, "a-.-G-U-S-M-V-C"),
    (CoTType::GndMedicalveterinarytheater, "a-.-G-U-S-M-V-T"),
    (CoTType::GndSupply, "a-.-G-U-S-S"),
    (CoTType::GndSupplyclassi, "a-.-G-U-S-S-1"),
    (CoTType::GndSupplyclassicorps, "a-.-G-U-S-S-1-C"),
    (CoTType::GndSupplyclassitheater, "a-.-G-U-S-S-1-T"),
    (CoTType::GndSupplyclassii, "a-.-G-U-S-S-2"),
    (CoTType::GndSupplyclassiicorps, "a-.-G-U-S-S-2-C"),
    (CoTType::GndSupplyclassiitheater, "a-.-G-U-S-S-2-T"),
    (CoTType::GndSupplyclassiii, "a-.-G-U-S-S-3"),
    (CoTType::GndSupplyclassiiiaviation, "a-.-G-U-S-S-3-A"),
    (CoTType::GndSupplyclassiiiaviationcorps, "a-.-G-U-S-S-3-A-C"),
    (
        CoTType::GndSupplyclassiiiaviationtheater,
        "a-.-G-U-S-S-3-A-T",
    ),
    (CoTType::GndSupplyclassiiicorps, "a-.-G-U-S-S-3-C"),
    (CoTType::GndSupplyclassiiitheater, "a-.-G-U-S-S-3-T"),
    (CoTType::GndSupplyclassiv, "a-.-G-U-S-S-4"),
    (CoTType::GndSupplyclassivcorps, "a-.-G-U-S-S-4-C"),
    (CoTType::GndSupplyclassivtheater, "a-.-G-U-S-S-4-T"),
    (CoTType::GndSupplyclassv, "a-.-G-U-S-S-5"),
    (CoTType::GndSupplyclassvcorps, "a-.-G-U-S-S-5-C"),
    (CoTType::GndSupplyclassvtheater, "a-.-G-U-S-S-5-T"),
    (CoTType::GndSupplyclassvi, "a-.-G-U-S-S-6"),
    (CoTType::GndSupplyclassivcorps, "a-.-G-U-S-S-6-C"),
    (CoTType::GndSupplyclassvitheater, "a-.-G-U-S-S-6-T"),
    (CoTType::GndSupplyclassvii, "a-.-G-U-S-S-7"),
    (CoTType::GndSupplyclassviicorps, "a-.-G-U-S-S-7-C"),
    (CoTType::GndSupplyclassviitheater, "a-.-G-U-S-S-7-T"),
    (CoTType::GndSupplyclassviii, "a-.-G-U-S-S-8"),
    (CoTType::GndSupplyclassviiicorps, "a-.-G-U-S-S-8-C"),
    (CoTType::GndSupplyclassviiitheater, "a-.-G-U-S-S-8-T"),
    (CoTType::GndSupplyclassix, "a-.-G-U-S-S-9"),
    (CoTType::GndSupplyclassixcorps, "a-.-G-U-S-S-9-C"),
    (CoTType::GndSupplyclassixtheater, "a-.-G-U-S-S-9-T"),
    (CoTType::GndSupplycorps, "a-.-G-U-S-S-C"),
    (CoTType::GndSupplylaundryBath, "a-.-G-U-S-S-L"),
    (CoTType::GndSupplylaundryBathcorps, "a-.-G-U-S-S-L-C"),
    (CoTType::GndSupplylaundryBaththeater, "a-.-G-U-S-S-L-T"),
    (CoTType::GndSupplytheater, "a-.-G-U-S-S-T"),
    (CoTType::GndSupplywater, "a-.-G-U-S-S-W"),
    (CoTType::GndSupplywatercorps, "a-.-G-U-S-S-W-C"),
    (CoTType::GndSupplywaterpurification, "a-.-G-U-S-S-W-P"),
    (
        CoTType::GndSupplywaterpurificationcorps,
        "a-.-G-U-S-S-W-P-C",
    ),
    (
        CoTType::GndSupplywaterpurificationtheater,
        "a-.-G-U-S-S-W-P-T",
    ),
    (CoTType::GndSupplywatertheater, "a-.-G-U-S-S-W-T"),
    (CoTType::GndSupplyclassx, "a-.-G-U-S-S-X"),
    (CoTType::GndSupplyclassxcorps, "a-.-G-U-S-S-X-C"),
    (CoTType::GndSupplyclassxtheater, "a-.-G-U-S-S-X-T"),
    (CoTType::GndSupplydepot, "a-.-G-U-S-S-d"),
    (CoTType::GndTransportation, "a-.-G-U-S-T"),
    (CoTType::GndApodApoe, "a-.-G-U-S-T-A"),
    (CoTType::GndApodApoecorps, "a-.-G-U-S-T-A-C"),
    (CoTType::GndApodApoetheater, "a-.-G-U-S-T-A-T"),
    (CoTType::GndTransportationcorps, "a-.-G-U-S-T-C"),
    (CoTType::GndMissile, "a-.-G-U-S-T-I"),
    (CoTType::GndMissilecorps, "a-.-G-U-S-T-I-C"),
    (CoTType::GndMissiletheater, "a-.-G-U-S-T-I-T"),
    (CoTType::GndMovementcontrolcenterMcc, "a-.-G-U-S-T-M"),
    (CoTType::GndMcccorps, "a-.-G-U-S-T-M-C"),
    (CoTType::GndMcctheater, "a-.-G-U-S-T-M-T"),
    (CoTType::GndRailhead, "a-.-G-U-S-T-R"),
    (CoTType::GndRailheadcorps, "a-.-G-U-S-T-R-C"),
    (CoTType::GndRailheadtheater, "a-.-G-U-S-T-R-T"),
    (CoTType::GndSpodSpoe, "a-.-G-U-S-T-S"),
    (CoTType::GndSpodSpoecorps, "a-.-G-U-S-T-S-C"),
    (CoTType::GndSpodSpoetheater, "a-.-G-U-S-T-S-T"),
    (CoTType::GndTransportationtheater, "a-.-G-U-S-T-T"),
    (CoTType::GndMaintenance, "a-.-G-U-S-X"),
    (CoTType::GndMaintenancecorps, "a-.-G-U-S-X-C"),
    (CoTType::GndElectrooptical, "a-.-G-U-S-X-E"),
    (CoTType::GndElectroopticalcorps, "a-.-G-U-S-X-E-C"),
    (CoTType::GndElectroopticaltheater, "a-.-G-U-S-X-E-T"),
    (CoTType::GndMaintenanceheavy, "a-.-G-U-S-X-H"),
    (CoTType::GndMaintenanceheavycorps, "a-.-G-U-S-X-H-C"),
    (CoTType::GndMaintenanceheavytheater, "a-.-G-U-S-X-H-T"),
    (CoTType::GndOrdnance, "a-.-G-U-S-X-O"),
    (CoTType::GndOrdnancecorps, "a-.-G-U-S-X-O-C"),
    (CoTType::GndOrdnancemissile, "a-.-G-U-S-X-O-M"),
    (CoTType::GndOrdnancemissilecorps, "a-.-G-U-S-X-O-M-C"),
    (CoTType::GndOrdnancemissiletheater, "a-.-G-U-S-X-O-M-T"),
    (CoTType::GndOrdnancetheater, "a-.-G-U-S-X-O-T"),
    (CoTType::GndMaintenancerecovery, "a-.-G-U-S-X-R"),
    (CoTType::GndMaintenancerecoverycorps, "a-.-G-U-S-X-R-C"),
    (CoTType::GndMaintenancerecoverytheater, "a-.-G-U-S-X-R-T"),
    (CoTType::GndMaintenancetheater, "a-.-G-U-S-X-T"),
    (CoTType::GndCombatsupport, "a-.-G-U-U"),
    (CoTType::GndCombatsupportnbc, "a-.-G-U-U-A"),
    (CoTType::GndBiological, "a-.-G-U-U-A-B"),
    (CoTType::GndReconequipped, "a-.-G-U-U-A-B-R"),
    (CoTType::GndChemical, "a-.-G-U-U-A-C"),
    (CoTType::GndSmokeDecon, "a-.-G-U-U-A-C-C"),
    (CoTType::GndMechanizedsmokeDecon, "a-.-G-U-U-A-C-C-K"),
    (CoTType::GndMotorizedsmokeDecon, "a-.-G-U-U-A-C-C-M"),
    (CoTType::GndChemicalrecon, "a-.-G-U-U-A-C-R"),
    (
        CoTType::GndChemicalwheeledarmoredvehicle,
        "a-.-G-U-U-A-C-R-S",
    ),
    (CoTType::GndSmoke, "a-.-G-U-U-A-C-S"),
    (CoTType::GndArmorsmoke, "a-.-G-U-U-A-C-S-A"),
    (CoTType::GndMotorizedsmoke, "a-.-G-U-U-A-C-S-M"),
    (CoTType::GndDecontamination, "a-.-G-U-U-A-D"),
    (CoTType::GndNuclear, "a-.-G-U-U-A-N"),
    (CoTType::GndExplosiveordinancedisposal, "a-.-G-U-U-E"),
    (CoTType::GndInformationwarfareunit, "a-.-G-U-U-I"),
    (CoTType::GndLawenforcementunit, "a-.-G-U-U-L"),
    (CoTType::GndCivilianlawenforcement, "a-.-G-U-U-L-C"),
    (CoTType::GndCentralintelligencedivisionCid, "a-.-G-U-U-L-D"),
    (CoTType::GndSecuritypoliceAir, "a-.-G-U-U-L-F"),
    (CoTType::GndMilitarypolice, "a-.-G-U-U-L-M"),
    (CoTType::GndShorepatrol, "a-.-G-U-U-L-S"),
    (CoTType::GndMilitaryintelligence, "a-.-G-U-U-M"),
    (CoTType::GndAerialexploitation, "a-.-G-U-U-M-A"),
    (CoTType::GndCounterintelligence, "a-.-G-U-U-M-C"),
    (CoTType::GndJointintelligencecenter, "a-.-G-U-U-M-J"),
    (CoTType::GndMeteorological, "a-.-G-U-U-M-M-O"),
    (CoTType::GndOperations, "a-.-G-U-U-M-O"),
    (CoTType::GndInterrogation, "a-.-G-U-U-M-Q"),
    (CoTType::GndSurveillance, "a-.-G-U-U-M-R"),
    (CoTType::GndGroundsurveillanceradar, "a-.-G-U-U-M-R-G"),
    (CoTType::GndSensor, "a-.-G-U-U-M-R-S"),
    (CoTType::GndSensorscm, "a-.-G-U-U-M-R-S-S"),
    (CoTType::GndGroundstationmodule, "a-.-G-U-U-M-R-X"),
    (CoTType::GndSignalintelligenceSigint, "a-.-G-U-U-M-S"),
    (CoTType::GndElectronicwarfare, "a-.-G-U-U-M-S-E"),
    (CoTType::GndArmoredwheeledvehicle, "a-.-G-U-U-M-S-E-A"),
    (CoTType::GndCorps, "a-.-G-U-U-M-S-E-C"),
    (CoTType::GndDirectionfinding, "a-.-G-U-U-M-S-E-D"),
    (CoTType::GndIntercept, "a-.-G-U-U-M-S-E-I"),
    (CoTType::GndJamming, "a-.-G-U-U-M-S-E-J"),
    (CoTType::GndTheater, "a-.-G-U-U-M-S-E-T"),
    (CoTType::GndTacticalexploit, "a-.-G-U-U-M-T"),
    (CoTType::GndLandingsupport, "a-.-G-U-U-P"),
    (CoTType::GndSignalunit, "a-.-G-U-U-S"),
    (CoTType::GndArea, "a-.-G-U-U-S-A"),
    (CoTType::GndCommunicationconfiguredpackage, "a-.-G-U-U-S-C"),
    (
        CoTType::GndLargecommunicationconfiguredpackage,
        "a-.-G-U-U-S-C-L",
    ),
    (CoTType::GndForwardcommunications, "a-.-G-U-U-S-F"),
    (CoTType::GndMultiplesubscriberelement, "a-.-G-U-U-S-M"),
    (CoTType::GndLargeextensionnode, "a-.-G-U-U-S-M-L"),
    (CoTType::GndNodecenter, "a-.-G-U-U-S-M-N"),
    (CoTType::GndSmallextensionnode, "a-.-G-U-U-S-M-S"),
    (CoTType::GndCommandoperations, "a-.-G-U-U-S-O"),
    (CoTType::GndRadiounit, "a-.-G-U-U-S-R"),
    (CoTType::GndTacticalsatellite, "a-.-G-U-U-S-R-S"),
    (CoTType::GndTeletypecenter, "a-.-G-U-U-S-R-T"),
    (CoTType::GndRelay, "a-.-G-U-U-S-R-W"),
    (CoTType::GndSignalsupport, "a-.-G-U-U-S-S"),
    (CoTType::GndTelephoneswitch, "a-.-G-U-U-S-W"),
    (CoTType::GndElectronicranging, "a-.-G-U-U-S-X"),
    (CoTType::GndImresources, "a-.-G-U-i"),
    (CoTType::GndImresourcesAnimalhealth, "a-.-G-U-i-a"),
    (CoTType::GndLargeanimalrescuestriketeam, "a-.-G-U-i-a-lar"),
    (CoTType::GndLargeanimalshelteringteam, "a-.-G-U-i-a-las"),
    (CoTType::GndLargeanimaltransportteam, "a-.-G-U-i-a-lat"),
    (CoTType::GndSmallanimalrescuestriketeam, "a-.-G-U-i-a-sar"),
    (CoTType::GndSmallanimalshelteringteam, "a-.-G-U-i-a-sas"),
    (CoTType::GndSmallanimaltransportteam, "a-.-G-U-i-a-sat"),
    (
        CoTType::GndIncidentmanagementteamanimalprotection,
        "a-.-G-U-i-a-imt",
    ),
    (CoTType::GndImresourcesEmergencymanagement, "a-.-G-U-i-e"),
    (
        CoTType::GndAirbornecommunicationsrelayCap,
        "a-.-G-U-i-e-acrc",
    ),
    (
        CoTType::GndAirbornecommunicationsrelayteam,
        "a-.-G-U-i-e-acrt",
    ),
    (CoTType::GndAirbornetransportteam, "a-.-G-U-i-e-att"),
    (
        CoTType::GndCriticalincidentstressmanagementteam,
        "a-.-G-U-i-e-cis",
    ),
    (CoTType::GndCommunicationssupportteam, "a-.-G-U-i-e-cst"),
    (CoTType::GndDonationscoordinator, "a-.-G-U-i-e-dc"),
    (
        CoTType::GndDonationsmanagementpersonnelteam,
        "a-.-G-U-i-e-dmt",
    ),
    (CoTType::GndEvacuationcoordinationteam, "a-.-G-U-i-e-ect"),
    (CoTType::GndEocfinanceadministration, "a-.-G-U-i-e-efa"),
    (CoTType::GndEvacuationliaisonteam, "a-.-G-U-i-e-elt"),
    (CoTType::GndEocmanagementsupportteam, "a-.-G-U-i-e-ems"),
    (CoTType::GndEocoperationssectionchief, "a-.-G-U-i-e-eos"),
    (CoTType::GndEocplanningsectionchief, "a-.-G-U-i-e-eps"),
    (
        CoTType::GndIndividualassistancedisasterassessmentteamleader,
        "a-.-G-U-i-e-ial",
    ),
    (
        CoTType::GndIndividualassistancedisasterassessmentteam,
        "a-.-G-U-i-e-iat",
    ),
    (CoTType::GndIncidentmanagementteam, "a-.-G-U-i-e-imt"),
    (CoTType::GndMobilecommunicationscenter, "a-.-G-U-i-e-mcc"),
    (CoTType::GndMobilefeedingkitchen, "a-.-G-U-i-e-mfk"),
    (CoTType::GndPublicassistancecoordinator, "a-.-G-U-i-e-pac"),
    (CoTType::GndRapidneedsassessmentteam, "a-.-G-U-i-e-rna"),
    (CoTType::GndSheltermanagementteam, "a-.-G-U-i-e-smt"),
    (CoTType::GndVolunteeragencyliaison, "a-.-G-U-i-e-val"),
    (
        CoTType::GndImresourcesEmergencymedicalservices,
        "a-.-G-U-i-m",
    ),
    (CoTType::GndAirambulanceFixedWing, "a-.-G-U-i-m-aaf"),
    (CoTType::GndAirambulanceRotaryWing, "a-.-G-U-i-m-aar"),
    (CoTType::GndAmbulancesGround, "a-.-G-U-i-m-ag"),
    (CoTType::GndAmbulancestriketeam, "a-.-G-U-i-m-ast"),
    (CoTType::GndAmbulancetaskforce, "a-.-G-U-i-m-atf"),
    (CoTType::GndEmergencymedicaltaskforce, "a-.-G-U-i-m-etf"),
    (CoTType::GndImresourcesFirehazmat, "a-.-G-U-i-f"),
    (CoTType::GndAreacommandteamFirefighting, "a-.-G-U-i-f-act"),
    (CoTType::GndBrushpatrol, "a-.-G-U-i-f-bp"),
    (CoTType::GndCrewtransport, "a-.-G-U-i-f-ct"),
    (CoTType::GndEngineFirePumper, "a-.-G-U-i-f-efp"),
    (CoTType::GndFireboat, "a-.-G-U-i-f-fb"),
    (CoTType::GndFueltender, "a-.-G-U-i-f-ft"),
    (CoTType::GndFiretruck, "a-.-G-U-i-f-fta"),
    (CoTType::GndFoamtender, "a-.-G-U-i-f-ftf"),
    (CoTType::GndHandcrew, "a-.-G-U-i-f-hc"),
    (CoTType::GndHazmatentryteam, "a-.-G-U-i-f-het"),
    (CoTType::GndHelicoptersfirefighting, "a-.-G-U-i-f-hf"),
    (CoTType::GndHelitanker, "a-.-G-U-i-f-ht"),
    (
        CoTType::GndIncidentmanagementteamfirefighting,
        "a-.-G-U-i-f-imt",
    ),
    (
        CoTType::GndInteragencybuyingteamfirefighting,
        "a-.-G-U-i-f-ibt",
    ),
    (CoTType::GndMobilecommunicationsunit, "a-.-G-U-i-f-mcu"),
    (CoTType::GndUscgnationalstrikeforce, "a-.-G-U-i-f-nsf"),
    (CoTType::GndPortablepump, "a-.-G-U-i-f-pp"),
    (CoTType::GndStriketeamEngineFire, "a-.-G-U-i-f-st"),
    (CoTType::GndWatertenderFirefightingTanker, "a-.-G-U-i-f-wt"),
    (CoTType::GndImresourcesHealthandmedical, "a-.-G-U-i-h"),
    (
        CoTType::GndDisastermedicalassistanceteamBasic,
        "a-.-G-U-i-h-dmb",
    ),
    (
        CoTType::GndDisastermedicalassistanceteamCrushinjuryspecialty,
        "a-.-G-U-i-h-dmc",
    ),
    (
        CoTType::GndDisastermedicalassistanceteamMentalhealth,
        "a-.-G-U-i-h-dmm",
    ),
    (
        CoTType::GndDisastermedicalassistanceteamPediatric,
        "a-.-G-U-i-h-dmp",
    ),
    (
        CoTType::GndDisastermedicalassistanceteamBurn,
        "a-.-G-U-i-h-dms",
    ),
    (
        CoTType::GndDisastermortuaryoperationalresponseteam,
        "a-.-G-U-i-h-mor",
    ),
    (
        CoTType::GndInternationalmedicalsurgicalresponseteam,
        "a-.-G-U-i-h-msr",
    ),
    (CoTType::GndNdmsmanagementsupportteam, "a-.-G-U-i-h-mst"),
    (
        CoTType::GndVeterinarymedicalassistanceteam,
        "a-.-G-U-i-h-vma",
    ),
    (CoTType::GndImresourcesLawenforcement, "a-.-G-U-i-l"),
    (CoTType::GndBombsquadexplosivesteam, "a-.-G-U-i-l-bs"),
    (CoTType::GndCrowdcontrolteam, "a-.-G-U-i-l-cct"),
    (CoTType::GndPublicsafetydiveteam, "a-.-G-U-i-l-dt"),
    (
        CoTType::GndAviationHelicopterspatrolsurveillance,
        "a-.-G-U-i-l-hps",
    ),
    (CoTType::GndObservationaircraft, "a-.-G-U-i-l-oa"),
    (CoTType::GndSwattacticalteam, "a-.-G-U-i-l-tt"),
    (CoTType::GndImresourcesPublicworks, "a-.-G-U-i-p"),
    (CoTType::GndAircurtainburnersAboveground, "a-.-G-U-i-p-acf"),
    (CoTType::GndAirconditionerheater, "a-.-G-U-i-p-ach"),
    (CoTType::GndAircurtainburnersInground, "a-.-G-U-i-p-act"),
    (CoTType::GndAllterraincranes, "a-.-G-U-i-p-atc"),
    (CoTType::GndBackhoeloader, "a-.-G-U-i-p-bh"),
    (CoTType::GndChillersairhandlers, "a-.-G-U-i-p-cah"),
    (CoTType::GndCrawlercranes, "a-.-G-U-i-p-cc"),
    (
        CoTType::GndConcretecuttermultiProcessorforhydraulicexcavator,
        "a-.-G-U-i-p-cce",
    ),
    (CoTType::GndDisasterassessmentteam, "a-.-G-U-i-p-dat"),
    (
        CoTType::GndDebrismanagementmonitoringteam,
        "a-.-G-U-i-p-dmm",
    ),
    (
        CoTType::GndDebrismanagementsitereductionteam,
        "a-.-G-U-i-p-dmr",
    ),
    (CoTType::GndDebrismanagementteam, "a-.-G-U-i-p-dmt"),
    (CoTType::GndDisasterrecoveryteam, "a-.-G-U-i-p-drt"),
    (CoTType::GndDumptrailer, "a-.-G-U-i-p-dt"),
    (CoTType::GndDumptruckoffroad, "a-.-G-U-i-p-dtf"),
    (CoTType::GndDumptruckonroad, "a-.-G-U-i-p-dtn"),
    (
        CoTType::GndElectricalpowerrestorationteam,
        "a-.-G-U-i-p-epr",
    ),
    (CoTType::GndEngineeringservices, "a-.-G-U-i-p-es"),
    (CoTType::GndFlatbedtrailertruck, "a-.-G-U-i-p-fbt"),
    (CoTType::GndGenerators, "a-.-G-U-i-p-gen"),
    (CoTType::GndHydraulicexcavatorLarge, "a-.-G-U-i-p-hel"),
    (CoTType::GndHydraulicexcavatorMedium, "a-.-G-U-i-p-hem"),
    (CoTType::GndHydraulictruckcranes, "a-.-G-U-i-p-htc"),
    (CoTType::GndLatticetruckcranes, "a-.-G-U-i-p-ltc"),
    (CoTType::GndTugboat, "a-.-G-U-i-p-tb"),
    (CoTType::GndTrackdozer, "a-.-G-U-i-p-td"),
    (CoTType::GndTubgrinder, "a-.-G-U-i-p-tg"),
    (CoTType::GndTractortrailer, "a-.-G-U-i-p-tt"),
    (CoTType::GndWheeldozer, "a-.-G-U-i-p-wd"),
    (CoTType::GndWheelloadersLarge, "a-.-G-U-i-p-wll"),
    (CoTType::GndWheelloadersMedium, "a-.-G-U-i-p-wlm"),
    (CoTType::GndWheelloadersSmall, "a-.-G-U-i-p-wls"),
    (CoTType::GndWaterpurificationteam, "a-.-G-U-i-p-wpt"),
    (CoTType::GndWatertruck, "a-.-G-U-i-p-wt"),
    (CoTType::GndImresourcesSearchandrescue, "a-.-G-U-i-s"),
    (CoTType::GndAirbornereconnaissance, "a-.-G-U-i-s-ar"),
    (CoTType::GndAirsearchteam, "a-.-G-U-i-s-ast"),
    (CoTType::GndCaninesarteamavalanchesnow, "a-.-G-U-i-s-cas"),
    (CoTType::GndCavesarteam, "a-.-G-U-i-s-cav"),
    (CoTType::GndCaninesarteamdisasterresponse, "a-.-G-U-i-s-cdr"),
    (CoTType::GndCaninesarteamlandcadaver, "a-.-G-U-i-s-clc"),
    (CoTType::GndCollapsesarteam, "a-.-G-U-i-s-col"),
    (CoTType::GndCaninesarteamwater, "a-.-G-U-i-s-cwa"),
    (CoTType::GndCaninesarteamwilderness, "a-.-G-U-i-s-cwi"),
    (
        CoTType::GndCaninesarteamwildernesstracking,
        "a-.-G-U-i-s-cwt",
    ),
    (CoTType::GndMinetunnelsarteam, "a-.-G-U-i-s-mt"),
    (CoTType::GndMountainsarteam, "a-.-G-U-i-s-mnt"),
    (CoTType::GndRadiodirectionfindingteam, "a-.-G-U-i-s-rdf"),
    (
        CoTType::GndSwiftwaterfloodsearchanddiverescueteam,
        "a-.-G-U-i-s-sfd",
    ),
    (CoTType::GndUsarincidentsupportteam, "a-.-G-U-i-s-uis"),
    (CoTType::GndUsartaskforce, "a-.-G-U-i-s-utf"),
    (CoTType::GndWildernesssarteam, "a-.-G-U-i-s-wi"),
    (CoTType::GndImresourcesOther, "a-.-G-U-i-o"),
    (CoTType::SurfaceSeasurfacetrack, "a-.-S"),
    (CoTType::SurfaceCombatant, "a-.-S-C"),
    (CoTType::SurfaceAmphibiouswarfareship, "a-.-S-C-A"),
    (CoTType::SurfaceAssaultvessel, "a-.-S-C-A-L-A"),
    (CoTType::SurfaceLandingcraft, "a-.-S-C-A-L-C"),
    (CoTType::SurfaceLandingship, "a-.-S-C-A-L-S"),
    (CoTType::SurfaceHovercraft, "a-.-S-C-H"),
    (CoTType::SurfaceLine, "a-.-S-C-L"),
    (CoTType::SurfaceBattleship, "a-.-S-C-L-B-B"),
    (CoTType::SurfaceCruiser, "a-.-S-C-L-C-C"),
    (CoTType::SurfaceCarrier, "a-.-S-C-L-C-V"),
    (CoTType::SurfaceDestroyer, "a-.-S-C-L-D-D"),
    (CoTType::SurfaceFrigateCorvette, "a-.-S-C-L-F-F"),
    (CoTType::SurfaceMinewarfarevessel, "a-.-S-C-M"),
    (CoTType::SurfaceMcmsupport, "a-.-S-C-M-M-A"),
    (CoTType::SurfaceMcmdrone, "a-.-S-C-M-M-D"),
    (CoTType::SurfaceMinehunter, "a-.-S-C-M-M-H"),
    (CoTType::SurfaceMinelayer, "a-.-S-C-M-M-L"),
    (CoTType::SurfaceMinesweeper, "a-.-S-C-M-M-S"),
    (CoTType::SurfacePatrol, "a-.-S-C-P"),
    (CoTType::SurfaceAntisubmarinewarfare, "a-.-S-C-P-S-B"),
    (CoTType::SurfaceAntisurfacewarfare, "a-.-S-C-P-S-U"),
    (CoTType::SurfaceNavygroup, "a-.-S-G"),
    (CoTType::SurfaceConvoy, "a-.-S-G-C"),
    (CoTType::SurfaceNavytaskgroup, "a-.-S-G-G"),
    (CoTType::SurfaceNavytaskforce, "a-.-S-G-T"),
    (CoTType::SurfaceNavytaskunit, "a-.-S-G-U"),
    (CoTType::SurfaceNoncombatant, "a-.-S-N"),
    (CoTType::SurfaceFleetsupport, "a-.-S-N-F"),
    (CoTType::SurfaceIntelligence, "a-.-S-N-I"),
    (CoTType::SurfaceHospitalship, "a-.-S-N-M"),
    (CoTType::SurfaceStation, "a-.-S-N-N"),
    (CoTType::SurfaceRescue, "a-.-S-N-N-R"),
    (CoTType::SurfaceUnderwayreplenishment, "a-.-S-N-R"),
    (CoTType::SurfaceServiceSupportharbor, "a-.-S-N-S"),
    (CoTType::SurfaceOwntrack, "a-.-S-O"),
    // (CoTType::SurfaceStation, "a-.-S-S"),
    (CoTType::SurfaceAswship, "a-.-S-S-A"),
    (CoTType::SurfacePicket, "a-.-S-S-P"),
    (CoTType::SurfaceNonmilitary, "a-.-S-X"),
    (CoTType::SurfaceFishing, "a-.-S-X-F"),
    (CoTType::SurfaceDrifter, "a-.-S-X-F-D-F"),
    (CoTType::SurfaceDredge, "a-.-S-X-F-D-R"),
    (CoTType::SurfaceTrawler, "a-.-S-X-F-T-R"),
    (CoTType::SurfaceLawenforcementvessel, "a-.-S-X-L"),
    (CoTType::SurfaceMerchant, "a-.-S-X-M"),
    (CoTType::SurfaceCargo, "a-.-S-X-M-C"),
    (CoTType::SurfaceFerry, "a-.-S-X-M-F"),
    (CoTType::SurfaceHazardousmaterialsHazmat, "a-.-S-X-M-H"),
    (CoTType::SurfaceOilerTanker, "a-.-S-X-M-O"),
    (CoTType::SurfacePassenger, "a-.-S-X-M-P"),
    (CoTType::SurfaceRollonRolloff, "a-.-S-X-M-R"),
    (CoTType::SurfaceTowingvessel, "a-.-S-X-M-T-O"),
    (CoTType::SurfaceTug, "a-.-S-X-M-T-U"),
    (CoTType::SurfaceLeisurecraft, "a-.-S-X-R"),
    (CoTType::SubsurfSubsurfacetrack, "a-.-U"),
    (CoTType::SubsurfNonsubmarine, "a-.-U-N"),
    (CoTType::SubsurfDiver, "a-.-U-N-D"),
    (CoTType::SubsurfSubmarine, "a-.-U-S"),
    (CoTType::SubsurfSubmarineconventionalpropulsion, "a-.-U-S-C"),
    (CoTType::SubsurfSubmarinenuclearpropulsion, "a-.-U-S-N"),
    (CoTType::SubsurfOthersubmersible, "a-.-U-S-O"),
    (CoTType::SubsurfStation, "a-.-U-S-S"),
    (CoTType::SubsurfAswsubmarine, "a-.-U-S-S-A"),
    (CoTType::SubsurfUnmannedunderwatervehicleUuv, "a-.-U-S-U"),
    (CoTType::SubsurfUnderwaterweapon, "a-.-U-W"),
    (CoTType::SubsurfUnderwaterdecoy, "a-.-U-W-D"),
    (CoTType::SubsurfSeaminedecoy, "a-.-U-W-D-M"),
    (CoTType::SubsurfSeamine, "a-.-U-W-M"),
    (CoTType::SubsurfSeaminedealt, "a-.-U-W-M-D"),
    (CoTType::SubsurfSeamineFloating, "a-.-U-W-M-F"),
    (CoTType::SubsurfSeamineFloatingDealt, "a-.-U-W-M-F-D"),
    (CoTType::SubsurfSeamineGround, "a-.-U-W-M-G"),
    (CoTType::SubsurfSeamineGroundDealt, "a-.-U-W-M-G-D"),
    (CoTType::SubsurfSeamineMoored, "a-.-U-W-M-M"),
    (CoTType::SubsurfSeamineMooredDealt, "a-.-U-W-M-M-D"),
    (CoTType::SubsurfSeamineInotherposition, "a-.-U-W-M-O"),
    (CoTType::SubsurfSeamineInotherpositionDealt, "a-.-U-W-M-O-D"),
    (CoTType::SubsurfTorpedo, "a-.-U-W-T"),
    (CoTType::Other, "a-.-X"),
    (CoTType::OtherIncident, "a-.-X-i"),
    (CoTType::OtherIncidentGeo, "a-.-X-i-g"),
    (CoTType::OtherIncidentGeoAvalanche, "a-.-X-i-g-a"),
    (CoTType::OtherIncidentGeoEarthquake, "a-.-X-i-g-e"),
    (CoTType::OtherIncidentGeoAftershock, "a-.-X-i-g-e-a"),
    (CoTType::OtherIncidentGeoEpicenter, "a-.-X-i-g-e-e"),
    (CoTType::OtherIncidentGeoLandslide, "a-.-X-i-g-l"),
    (CoTType::OtherIncidentGeoSubsistance, "a-.-X-i-g-s"),
    (CoTType::OtherIncidentGeoVolcano, "a-.-X-i-g-v"),
    (CoTType::OtherIncidentGeoEruption, "a-.-X-i-g-v-e"),
    (CoTType::OtherIncidentGeoVolcanothreat, "a-.-X-i-g-v-t"),
    (CoTType::OtherIncidentMet, "a-.-X-i-m"),
    (CoTType::OtherIncidentMetDrizzle, "a-.-X-i-m-z"),
    (CoTType::OtherIncidentMetDrought, "a-.-X-i-m-d"),
    (CoTType::OtherIncidentMetFlood, "a-.-X-i-m-f"),
    (CoTType::OtherIncidentMetFog, "a-.-X-i-m-g"),
    (CoTType::OtherIncidentMetHail, "a-.-X-i-m-h"),
    (CoTType::OtherIncidentMetInversion, "a-.-X-i-m-i"),
    (CoTType::OtherIncidentMetRain, "a-.-X-i-m-r"),
    (CoTType::OtherIncidentMetSandstorm, "a-.-X-i-m-a"),
    (CoTType::OtherIncidentMetSnow, "a-.-X-i-m-s"),
    (CoTType::OtherIncidentMetthunderstorm, "a-.-X-i-m-u"),
    (CoTType::OtherIncidentMetTornado, "a-.-X-i-m-t"),
    (CoTType::OtherIncidentMetCyclone, "a-.-X-i-m-c"),
    (CoTType::OtherIncidentMetTsunami, "a-.-X-i-m-n"),
    (CoTType::OtherIncidentSafety, "a-.-X-i-s"),
    (CoTType::OtherIncidentSecurity, "a-.-X-i-l"),
    (CoTType::OtherIncidentSecurityCivildisturb, "a-.-X-i-l-c"),
    (
        CoTType::OtherIncidentSecurityCivildisturbDemonstration,
        "a-.-X-i-l-c-d",
    ),
    (
        CoTType::OtherIncidentSecurityCivildisturbDisplaced,
        "a-.-X-i-l-c-p",
    ),
    (
        CoTType::OtherIncidentSecurityCivildisturbRioting,
        "a-.-X-i-l-c-r",
    ),
    (CoTType::OtherIncidentSecurityLawenforcement, "a-.-X-i-l-l"),
    (
        CoTType::OtherIncidentSecurityLawenforcementBombing,
        "a-.-X-i-l-l-b",
    ),
    (
        CoTType::OtherIncidentSecurityLawenforcementBombexplosion,
        "a-.-X-i-l-l-b-e",
    ),
    (
        CoTType::OtherIncidentSecurityLawenforcementbombthreat,
        "a-.-X-i-l-l-b-t",
    ),
    (
        CoTType::OtherIncidentSecurityLawenforcementLooting,
        "a-.-X-i-l-l-l",
    ),
    (
        CoTType::OtherIncidentSecurityLawenforcementPoisoning,
        "a-.-X-i-l-l-p",
    ),
    (
        CoTType::OtherIncidentSecurityLawenforcementShooting,
        "a-.-X-i-l-l-s",
    ),
    (CoTType::OtherIncidentRescue, "a-.-X-i-r"),
    (CoTType::OtherIncidentFire, "a-.-X-i-f"),
    (CoTType::OtherIncidentFireWild, "a-.-X-i-f-w"),
    (CoTType::OtherIncidentFireNonresidential, "a-.-X-i-f-n"),
    (
        CoTType::OtherIncidentFireNonresidentialspecialneed,
        "a-.-X-i-f-n-h",
    ),
    (
        CoTType::OtherIncidentFireNonresidentialschool,
        "a-.-X-i-f-n-s",
    ),
    (CoTType::OtherIncidentFireResidential, "a-.-X-i-f-r"),
    (CoTType::OtherIncidentFireHotspot, "a-.-X-i-f-h"),
    (CoTType::OtherIncidentFireOrigin, "a-.-X-i-f-o"),
    (CoTType::OtherIncidentFireSmoke, "a-.-X-i-f-s"),
    (CoTType::OtherIncidentFireSpecialneeds, "a-.-X-i-f-p"),
    (CoTType::OtherIncidentMedicalpublichealth, "a-.-X-i-h"),
    (CoTType::OtherIncidentPollutionenvironmental, "a-.-X-i-e"),
    (CoTType::OtherIncidentTransportation, "a-.-X-i-t"),
    (CoTType::OtherIncidentTransportationAir, "a-.-X-i-t-a"),
    (
        CoTType::OtherIncidentTransportationAiraccident,
        "a-.-X-i-t-a-a",
    ),
    (
        CoTType::OtherIncidentTransportationAirhijacking,
        "a-.-X-i-t-a-h",
    ),
    (CoTType::OtherIncidentTransportationMaritime, "a-.-X-i-t-m"),
    (
        CoTType::OtherIncidentTransportationMaritimeaccident,
        "a-.-X-i-t-m-a",
    ),
    (
        CoTType::OtherIncidentTransportationMaritimehijacking,
        "a-.-X-i-t-m-h",
    ),
    (CoTType::OtherIncidentTransportationRail, "a-.-X-i-t-r"),
    (
        CoTType::OtherIncidentTransportationRailaccident,
        "a-.-X-i-t-r-a",
    ),
    (
        CoTType::OtherIncidentTransportationRailhijacking,
        "a-.-X-i-t-r-h",
    ),
    (CoTType::OtherIncidentTransportationVehicle, "a-.-X-i-t-v"),
    (
        CoTType::OtherIncidentTransportationVehicleaccident,
        "a-.-X-i-t-v-a",
    ),
    (
        CoTType::OtherIncidentTransportationVehiclehijacking,
        "a-.-X-i-t-v-h",
    ),
    (CoTType::OtherIncidentInfrastructure, "a-.-X-i-i"),
    (CoTType::OtherIncidentCbrne, "a-.-X-i-c"),
    (CoTType::OtherIncidentOther, "a-.-X-i-o"),
    (CoTType::Maydaymessage, "a-.-.*-9-1-1"),
];

/// A function to lookup the string from the CotDefinitions array
pub fn lookup_cot_type(cot_type: CoTType, cot_classification: CotClassification) -> &'static str {
    let char_classification = match cot_classification {
        CotClassification::Pending => "p",
        CotClassification::Unknown => "u",
        CotClassification::AssumedFriend => "a",
        CotClassification::Friend => "f",
        CotClassification::Neutral => "n",
        CotClassification::Suspect => "s",
        CotClassification::Hostile => "h",
        CotClassification::Joker => "j",
        CotClassification::Faker => "k",
        CotClassification::NoneSpecified => "o",
        CotClassification::Other => "x",
    };

    for (t, s) in COT_DEFINITIONS.iter() {
        if *t == cot_type {
            // Replace the '.' with 'f'
            let updated = s.replace(".", char_classification);
            return Box::leak(updated.into_boxed_str());
        }
    }
    "a-.-X-i-o"
}

/// Function to convert enum to string on classification
pub fn cot_classification_to_string(cot_classification: CotClassification) -> &'static str {
    match cot_classification {
        CotClassification::Pending => "Pending",
        CotClassification::Unknown => "Unknown",
        CotClassification::AssumedFriend => "Assumed Friend",
        CotClassification::Friend => "Friend",
        CotClassification::Neutral => "Neutral",
        CotClassification::Suspect => "Suspect",
        CotClassification::Hostile => "Hostile",
        CotClassification::Joker => "Joker",
        CotClassification::Faker => "Faker",
        CotClassification::NoneSpecified => "None Specified",
        CotClassification::Other => "Other",
    }
}
