// Code generated by protoc-gen-validate. DO NOT EDIT.
// source: pkg/apis/errordetails/v2/errordetails.proto

package errordetails

import (
	"bytes"
	"errors"
	"fmt"
	"net"
	"net/mail"
	"net/url"
	"regexp"
	"sort"
	"strings"
	"time"
	"unicode/utf8"

	"google.golang.org/protobuf/types/known/anypb"
)

// ensure the imports are used
var (
	_ = bytes.MinRead
	_ = errors.New("")
	_ = fmt.Print
	_ = utf8.UTFMax
	_ = (*regexp.Regexp)(nil)
	_ = (*strings.Reader)(nil)
	_ = net.IPv4len
	_ = time.Duration(0)
	_ = (*url.URL)(nil)
	_ = (*mail.Address)(nil)
	_ = anypb.Any{}
	_ = sort.Sort
)

// Validate checks the field values on Backend with the rules defined in the
// proto definition for this message. If any rules are violated, the first
// error encountered is returned, or nil if there are no violations.
func (m *Backend) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on Backend with the rules defined in the
// proto definition for this message. If any rules are violated, the result is
// a list of violation errors wrapped in BackendMultiError, or nil if none found.
func (m *Backend) ValidateAll() error {
	return m.validate(true)
}

func (m *Backend) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	// no validation rules for Message

	// no validation rules for Header

	if m.StatusCode != nil {

		if m.GetStatusCode() != 0 {

			if val := m.GetStatusCode(); val < 100 || val >= 599 {
				err := BackendValidationError{
					field:  "StatusCode",
					reason: "value must be inside range [100, 599)",
				}
				if !all {
					return err
				}
				errors = append(errors, err)
			}

		}

	}

	if len(errors) > 0 {
		return BackendMultiError(errors)
	}

	return nil
}

// BackendMultiError is an error wrapping multiple validation errors returned
// by Backend.ValidateAll() if the designated constraints aren't met.
type BackendMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m BackendMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m BackendMultiError) AllErrors() []error { return m }

// BackendValidationError is the validation error returned by Backend.Validate
// if the designated constraints aren't met.
type BackendValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e BackendValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e BackendValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e BackendValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e BackendValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e BackendValidationError) ErrorName() string { return "BackendValidationError" }

// Error satisfies the builtin error interface
func (e BackendValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sBackend.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = BackendValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = BackendValidationError{}

// Validate checks the field values on Unknown with the rules defined in the
// proto definition for this message. If any rules are violated, the first
// error encountered is returned, or nil if there are no violations.
func (m *Unknown) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on Unknown with the rules defined in the
// proto definition for this message. If any rules are violated, the result is
// a list of violation errors wrapped in UnknownMultiError, or nil if none found.
func (m *Unknown) ValidateAll() error {
	return m.validate(true)
}

func (m *Unknown) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	if m.Message != nil {
		// no validation rules for Message
	}

	if len(errors) > 0 {
		return UnknownMultiError(errors)
	}

	return nil
}

// UnknownMultiError is an error wrapping multiple validation errors returned
// by Unknown.ValidateAll() if the designated constraints aren't met.
type UnknownMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m UnknownMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m UnknownMultiError) AllErrors() []error { return m }

// UnknownValidationError is the validation error returned by Unknown.Validate
// if the designated constraints aren't met.
type UnknownValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e UnknownValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e UnknownValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e UnknownValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e UnknownValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e UnknownValidationError) ErrorName() string { return "UnknownValidationError" }

// Error satisfies the builtin error interface
func (e UnknownValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sUnknown.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = UnknownValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = UnknownValidationError{}
