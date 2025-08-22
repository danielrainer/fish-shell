include(FeatureSummary)

set(SPHINX_ROOT_DIR "${CMAKE_CURRENT_BINARY_DIR}/user_doc")

set(FISH_INDENT_FOR_BUILDING_DOCS "" CACHE FILEPATH "Path to fish_indent executable for building HTML docs")

if(FISH_INDENT_FOR_BUILDING_DOCS)
    set(SPHINX_HTML_FISH_INDENT_DEP)
else()
    set(FISH_INDENT_FOR_BUILDING_DOCS "${CMAKE_CURRENT_BINARY_DIR}/fish_indent")
    set(SPHINX_HTML_FISH_INDENT_DEP fish_indent)
endif()

add_custom_target(sphinx-docs
    COMMAND env ${VARS_FOR_CARGO}
        cargo xtask html-docs --fish-indent=${FISH_INDENT_FOR_BUILDING_DOCS}
    COMMAND mkdir -p ${SPHINX_ROOT_DIR}
    COMMAND cp -r ${CMAKE_CURRENT_BINARY_DIR}/cargo/build/fish-docs/html ${SPHINX_ROOT_DIR}
    DEPENDS ${SPHINX_HTML_FISH_INDENT_DEP}
)

add_custom_target(sphinx-manpages
    COMMAND env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}/cargo/build/
        cargo xtask man-pages
    COMMAND mkdir -p ${SPHINX_ROOT_DIR}
    COMMAND cp -r ${CMAKE_CURRENT_BINARY_DIR}/cargo/build/fish-docs/man ${SPHINX_ROOT_DIR}
)

if(NOT DEFINED WITH_DOCS) # Don't check for legacy options if the new one is defined, to help bisecting.
    if(DEFINED BUILD_DOCS)
        message(FATAL_ERROR "the BUILD_DOCS option is no longer supported, use -DWITH_DOCS=ON|OFF")
    endif()
    if(DEFINED INSTALL_DOCS)
        message(FATAL_ERROR "the INSTALL_DOCS option is no longer supported, use -DWITH_DOCS=ON|OFF")
    endif()
endif()

if(SPHINX_EXECUTABLE)
    option(WITH_DOCS "build documentation (requires Sphinx)" ON)
else()
    option(WITH_DOCS "build documentation (requires Sphinx)" OFF)
endif()

if(WITH_DOCS AND NOT SPHINX_EXECUTABLE)
    message(FATAL_ERROR "build documentation selected, but sphinx-build could not be found")
endif()

add_feature_info(Documentation WITH_DOCS "user manual and documentation")

if(WITH_DOCS)
    add_custom_target(doc ALL
                      DEPENDS sphinx-docs sphinx-manpages)
    # Group docs targets into a DocsTargets folder
    set_property(TARGET doc sphinx-docs sphinx-manpages
                 PROPERTY FOLDER cmake/DocTargets)
endif()
